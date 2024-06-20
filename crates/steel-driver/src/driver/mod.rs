use crate::lexer::Lexer;
use std::{
    fs::File,
    io::{Read, Result},
    path::{Path, PathBuf},
};

pub mod module;
use module::Module;

#[derive(Clone, Debug)]
pub struct Driver {
    pub global: Module,
}

impl Driver {
    pub fn new() -> Self {
        Self {
            global: Module {
                name: "<global>".to_string(),
                filename: "<global>".to_string(),
                source: "".to_string(),
                imports: Vec::new(),
                externs: Vec::new(),
                exports: Vec::new(),
                statements: Vec::new(),
                requires: Vec::new(),
            },
        }
    }

    pub fn compile(&mut self, child: String) -> Result<Module> {
        let mut module = Module::default();

        let mut file = File::open(child.clone()).expect("Unable to open file");
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)?;

        let mut lexer = Lexer::<8, 16>::new();
        let tokens = lexer.lex(file_contents.clone());

        module.source = file_contents;
        module.filename = child;
        module.name = {
            let name = PathBuf::from(module.filename.clone())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let name = name.split('.');

            name.clone().take(name.count() - 1).collect()
        };
        self.global.requires.push(module.clone());

        for tokens_chunk in tokens.chunks(3) {
            for token in tokens_chunk {
                println!("{:?}", token);
            }
            println!();
        }

        Ok(module)
    }
}
