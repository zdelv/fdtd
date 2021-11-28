use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not calculate courant or dt")]
    CourantError(String)
}