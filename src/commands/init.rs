use pots::Garden;
use error::*;

#[derive(Debug, StructOpt)]
pub struct Args {
}

pub fn command(_args: &Args) -> Result<()> {
    Garden::init(".")?;
    Ok(())
}
