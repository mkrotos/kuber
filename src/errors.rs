use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error when executing kubectl command {0}")]
    KubecltNotFound(#[from] io::Error),
    #[error("error parsing kubectl output")]
    ParseOutputError,
}
