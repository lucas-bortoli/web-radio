local function table_contains(tbl, item)
    for _, v in ipairs(tbl) do
        if v == item then
            return true
        end
    end
    return false
end

------------------------------------------------------------------------------

-- Generates pseudo-random numbers.
--
-- @param seed The initial seed value.
-- @return A function that returns a pseudo-random number between 0 and 1.
local function RNGAwful(seed)
    local a = 1664525
    local c = 1013904223
    local m = 2 ^ 32

    local state = seed

    return function()
        state = (a * state + c) % m
        local roll = state / m -- Normalize to [0, 1)
        return roll
    end
end

------------------------------------------------------------------------------

function SlidingWindow(capacity)
    local sliding_window = {}

    sliding_window.capacity = capacity
    sliding_window.contents = {}

    function sliding_window:append(item)
        while #self.contents >= self.capacity do
            table.remove(self.contents, 1) -- removes first item (oldest one)
        end
        table.insert(self.contents, item)
    end

    return sliding_window
end

------------------------------------------------------------------------------

local tracks = {
    {
        title = "Way Back Home",
        artist = "Bing Crosby",
        duration = 10
    }
}

local STATION_SEED = 5000
local STATION_EPOCH = 1745588382
local SILENCE_INTERVAL = 1
local SLIDING_WINDOW_SIZE = 8

if #tracks <= SLIDING_WINDOW_SIZE then
    error("Não há tracks suficientes para fazer o sliding window transbordar!")
end

local States = {
    NarrationBefore = { narration = nil, imminent_track = nil },
    Track = { track = nil },
    NarrationAfter = { previous_track = nil, narration = nil },
    SilenceInterval = {},
}

function pick_next()
    local rng = RNGAwful(STATION_SEED)
    local previous_tracks = SlidingWindow(SLIDING_WINDOW_SIZE)

    local state = States.SilenceInterval
    local current_track = nil

    local elapsed = os.time() - STATION_EPOCH -- how many seconds since the station epoch?
    local step_duration = 0

    local function pick_next_track_no_repetition()
        while true do
            local chosen_idx = math.floor(rng() * #tracks) + 1
            local chosen_track = tracks[chosen_idx]

            if not table_contains(previous_tracks.contents, chosen_track) then
                return chosen_track
            end
        end
    end

    -- determine expected station state from current time
    while true do
        local next_state

        if state == States.NarrationBefore then
            States.Track.track = state.imminent_track
            next_state = States.Track
            if state.narration == nil then
                step_duration = 0
            else
                step_duration = 0 -- <---- TODO: narration duration
            end
        elseif state == States.Track then
            States.NarrationAfter.previous_track = state.track
            States.NarrationAfter.narration = nil -- TODO: pick random ending narration
            next_state = States.NarrationAfter
            step_duration = state.track.duration
        elseif state == States.NarrationAfter then
            next_state = States.SilenceInterval
            if state.narration == nil then
                step_duration = 0
            else
                step_duration = 0 -- <---- TODO: narration duration
            end
        elseif state == States.SilenceInterval then
            States.NarrationBefore.imminent_track = pick_next_track_no_repetition()
            States.NarrationBefore.narration = nil -- TODO: track narrations

            next_state = States.NarrationBefore
            step_duration = SILENCE_INTERVAL
        end

        if step_duration < elapsed then
            -- subtract time accumulator and keep going
            state = next_state
            elapsed = elapsed - step_duration
        else
            -- we've found the expected state!
            break
        end
    end

    -- return state, elapsed

    return state
end
