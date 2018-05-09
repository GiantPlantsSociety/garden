use error::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{Read, Write};

pub fn process_bytes_and_display_progress<R: Read, W: Write>(inp: &mut R, out: &mut W, bytes_total: u64) -> Result<()> {
    let pb = ProgressBar::new(bytes_total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    process_bytes(inp, out, Some(&pb))?;
    pb.finish_and_clear();
    Ok(())
}

pub fn process_bytes<R: Read, W: Write>(inp: &mut R, out: &mut W, pb: Option<&ProgressBar>) -> Result<()> {
    let mut buffer = [0; 128 * 1024];
    let mut bytes_read = 0;
    loop {
        let len = inp.read(&mut buffer).map_err(Error::Io)?;
        if len == 0 {
            break;
        }

        out.write_all(&buffer[..len]).map_err(Error::Io)?;
        bytes_read += len;

        pb.map(|pb| pb.set_position(bytes_read as u64));
    }
    Ok(())
}
