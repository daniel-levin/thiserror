use thiserror::Error;

#[derive(Error, Debug)]
#[error("...")]
pub struct LargeError {
    a: [u8; 2048],
}

#[derive(Error, Debug)]
#[error("...")]
pub enum Autoboxed {
    Large(
        #[from]
        #[box_from]
        LargeError,
    ),
}

fn direct_return_large() -> Result<(), LargeError> {
    Err(LargeError { a: [0; 2048] })
}

/*
fn autobox() -> Result<(), Box<Autoboxed>> {
    let _ = direct_return_large()?;

    Ok(())
}
*/
