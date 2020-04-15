use std::io;
use io::Read;
use io::{Seek, SeekFrom};

pub trait ReaderExt: Read + Seek {
    fn read_bytes(&mut self, bytes: usize)
        -> Result<Vec<u8>, io::Error>
    {
        let mut buf = Vec::with_capacity(bytes);

        self
            .take(bytes as u64)
            .read_to_end(&mut buf)
            .and_then(|_| match buf.len() {
                l if l == bytes => Ok(buf),
                _ => Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    format!("Found EoF before being able to read {} bytes", bytes)
                )),
            })
    }
    
    fn read_bytes_to_string(&mut self, bytes: usize)
        -> Result<String, io::Error>
    {
        self.read_bytes(bytes)
            .map(|v| String::from_utf8_lossy(&v).to_string())
    }

    fn read_u32(&mut self)
        -> Result<u32, io::Error>
    {
        let mut buf = [0; 4];

        self.read_exact(&mut buf)
            .map(|_| u32::from_le_bytes(buf))
    }

    fn read_u16(&mut self)
        -> Result<u16, io::Error>
    {
        let mut buf = [0; 2];

        self.read_exact(&mut buf)
            .map(|_| u16::from_le_bytes(buf))
    }

    fn read_f32(&mut self)
        -> Result<f32, io::Error>
    {
        let mut buf = [0; 4];

        self.read_exact(&mut buf)
            .map(|_| f32::from_le_bytes(buf))
    }

    fn seek_from_current(&mut self, i: i64)
        -> Result<u64, io::Error>
    {
        self.seek(SeekFrom::Current(i))
    }

    fn seek_from_start(&mut self, i: u64)
        -> Result<u64, io::Error>
    {
        self.seek(SeekFrom::Start(i))
    }
}

impl<R: Read + Seek> ReaderExt for R {}

#[cfg(test)]
mod test
{
    use super::*;
    use std::io::Cursor;
    
    #[test]
    fn valid_read_bytes_to_string() {
        let data = "12345";
        let mut c = Cursor::new(data.as_bytes());

        let val = c.read_bytes_to_string(3).unwrap();

        assert_eq!("123", &val);
    }

    #[test]
    fn invalid_read_bytes_to_string() {
        let data = "123";
        let mut c = Cursor::new(data.as_bytes());

        let err = c.read_bytes_to_string(5).unwrap_err();

        assert_eq!(io::ErrorKind::UnexpectedEof, err.kind());
        // assert!(false);
    }

    #[test]
    fn valid_read_u16() {
        let num: u16 = 10;
        let mut c = Cursor::new(num.to_le_bytes());

        let val = c.read_u16().unwrap();

        assert_eq!(num, val);
    }

    #[test]
    fn valid_read_f32() {
        let num: f32 = 10.2424;
        let mut c = Cursor::new(num.to_le_bytes());

        let val = c.read_f32().unwrap();

        assert_eq!(num, val);
    }
    
    #[test]
    fn valid_read_u32() {
        let num: u32 = 10;
        let mut c = Cursor::new(num.to_le_bytes());

        let val = c.read_u32().unwrap();

        assert_eq!(num, val);
    }

    #[test]
    fn invalid_read_u32() {
        let num: u8 = 1;
        let mut c = Cursor::new(num.to_le_bytes());

        let err = c.read_u32().unwrap_err();

        assert_eq!(io::ErrorKind::UnexpectedEof, err.kind());
    }

    #[test]
    fn valid_seek_from_current() {
        let data = "1234567890";
        let mut c = Cursor::new(data.as_bytes());
        let mut s = String::new();

        c.seek_from_current(3).unwrap();
        c.read_to_string(&mut s).unwrap();

        assert_eq!("4567890", &s);
    }

    #[test]
    fn invalid_seek_from_current() {
        let data = "1234567890";
        let mut c = Cursor::new(data.as_bytes());

        let err = c.seek_from_current(-1).unwrap_err();

        assert_eq!(io::ErrorKind::InvalidInput, err.kind());
    }
}