use std::path::Path;

use crate::init::{InitError, run_init_with_shell_setup};

pub fn execute(root: &Path, prefix: &Path) -> Result<(), zb_core::Error> {
    run_init_with_shell_setup(root, prefix).map_err(|e| match e {
        InitError::Message(msg) => zb_core::Error::StoreCorruption { message: msg },
    })
}
