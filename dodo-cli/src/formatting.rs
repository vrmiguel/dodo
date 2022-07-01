use std::io::Write;
use std::path::Path;
use std::str;

use dodo_internals::chrono::NaiveDate;

use crate::Result;

pub const FMT_STRING: &str = "%Y-%m-%d";
pub const FMT_STRING_WITH_EXT: &str = "%Y-%m-%d.bin";

/// Enough bytes to hold dates in the format "YYYY-mm-dd.bin"
pub struct DateBuffer {
    inner: [u8; 14],
}

impl DateBuffer {
    pub fn new() -> Self {
        Self { inner: [0; 14] }
    }

    pub fn format_path(&mut self, date: NaiveDate) -> Result<&Path> {
        let fmt = date.format(FMT_STRING_WITH_EXT);
        write!(&mut self.inner[..], "{fmt}")?;

        // Safety: chrono's formatting with the given format string will
        // return ASCII-only, guaranteed to be padded to 14 bytes (unless we're over the year 9999)
        let utf8 = unsafe { str::from_utf8_unchecked(&self.inner) };

        Ok(Path::new(utf8))
    }
}

#[cfg(test)]
mod tests {
    use super::DateBuffer;

    #[test]
    fn date_buffer_has_enough_space_to_fit_formatted_date() {
        let buf = DateBuffer::new();

        assert_eq!(buf.inner.len(), "1999-10-22.bin".len());
    }
}
