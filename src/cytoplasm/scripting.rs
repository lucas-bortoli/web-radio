use std::{fs::File, io::Read, path::Path};

use mlua::{FromLua, Function, Lua};

pub struct Scripting {
    state: Lua,
}

impl Scripting {
    pub fn new(source_file: &Path) -> Scripting {
        let mut source_code = String::new();
        File::open(source_file)
            .expect("scripting: falha ao abrir arquivo")
            .read_to_string(&mut source_code)
            .expect("scripting: falha ao ler arquivo");

        let state = Lua::new();

        state
            .load(source_code)
            .set_mode(mlua::ChunkMode::Text)
            .set_name(source_file.file_name().map_or("=script".to_string(), |f| {
                "@".to_string() + f.to_str().unwrap()
            }))
            .exec()
            .expect("scripting: falha ao executar código");

        Scripting { state }
    }

    fn get_global<T: FromLua>(&self, name: &str) -> Option<T> {
        self.state
            .globals()
            .get(name)
            .map_or(None, |value| Some(value))
    }

    pub fn invoke_pick_next(&self) -> Option<String> {
        let fn_ref = self.get_global::<Function>("pick_next")?;
        let returned = fn_ref
            .call::<String>(())
            .expect("scripting: falha ao chamar função user");

        Some(returned)
    }
}
