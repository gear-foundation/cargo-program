pub mod extrinsics;

use std::path::PathBuf;

use anyhow::Result;
use rhai::Engine;

use crate::error::CrateError;

static mut CODE: Vec<u8> = Vec::new();
static mut NODE_URL: String = String::new();

pub fn set_code(code: Vec<u8>) {
    unsafe {
        CODE = code;
    }
}

pub fn set_node_url(url: String) {
    unsafe {
        NODE_URL = url;
    }
}

pub fn run(script: PathBuf) -> Result<()> {
    Runner::new(script).run()
}

#[derive(Debug, Default)]
pub struct Runner {
    engine: Engine,
    script: PathBuf,
}

impl Runner {
    pub fn new(script: PathBuf) -> Self {
        let mut engine = Engine::new();
        engine
            .register_type::<Program>()
            .register_fn("program", Program::from_code);

        Runner {
            engine,
            script,
        }
    }

    pub fn run(self) -> Result<()> {
        self.engine
            .run_file(self.script)
            .map_err(|e| CrateError::ScriptRunningError(e.to_string()).into())
    }
}

#[derive(Clone, Debug, Default)]
struct Program {
    code: Vec<u8>,
}

impl Program {
    fn from_code() -> Self {
        Program {
            code: unsafe { CODE.clone() }
        }
    }
}
