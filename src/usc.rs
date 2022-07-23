use std::{
    fmt,
    io::BufRead,
    mem,
    str::{self, SplitAsciiWhitespace},
};

/// Unsafe scanner.
pub struct USC<R> {
    reader: R,
    tokens: SplitAsciiWhitespace<'static>,
    bytes: Vec<u8>,
}

impl<R: BufRead> USC<R> {
    /// Creates a new scanner.
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            tokens: "".split_ascii_whitespace(),
            bytes: Vec::new(),
        }
    }

    /// Returns white-space separated tokens each time `read` is called.
    ///
    /// # Panic
    ///
    /// - Panics if read fails
    /// - Panics if the token fails to parse to `T`.
    pub fn read<T>(&mut self) -> T
    where
        T: str::FromStr,
        <T as str::FromStr>::Err: fmt::Debug,
    {
        loop {
            if let Some(token) = self.tokens.next() {
                return token.parse().unwrap();
            }

            let line = self.read_line();
            self.tokens = unsafe { mem::transmute(line.split_ascii_whitespace()) }
        }
    }

    /// Read and return a line from `self.reader`.
    ///
    /// # Panic
    ///
    /// Panics if read fails.
    pub fn read_line(&mut self) -> &str {
        self.bytes.clear();
        self.reader.read_until(b'\n', &mut self.bytes).unwrap();
        unsafe { str::from_utf8_unchecked(&self.bytes) }
    }
}

#[cfg(test)]
mod tests {
    use super::USC;
    use std::io::BufReader;

    #[test]
    fn read() {
        let buf = "1 2 3 4 5 4 3 2 1".as_bytes();
        let reader = BufReader::new(buf);
        let mut usc = USC::new(reader);

        assert_eq!(usc.read::<u8>(), 1);
        assert_eq!(usc.read::<u8>(), 2);
        assert_eq!(usc.read::<u8>(), 3);
        assert_eq!(usc.read::<u8>(), 4);
        assert_eq!(usc.read::<u8>(), 5);
        assert_eq!(usc.read::<u8>(), 4);
        assert_eq!(usc.read::<u8>(), 3);
        assert_eq!(usc.read::<u8>(), 2);
        assert_eq!(usc.read::<u8>(), 1);
    }

    #[test]
    fn read_line() {
        let buf = "1 2 3 4 5".as_bytes();
        let reader = BufReader::new(buf);
        let mut usc = USC::new(reader);

        assert_eq!(usc.read_line(), "1 2 3 4 5");
    }
}
