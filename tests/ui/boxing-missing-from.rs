use thiserror::Error;

#[derive(Error, Debug)]
pub enum Missing {
    #[error(transparent)]
    NotThere(#[boxing] std::io::Error),
}

fn main() {}
