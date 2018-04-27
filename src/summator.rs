use std::default::Default;
use std::io::{Write, Result as IoResult};
use std::convert::Into;
use md5;
use sha1::Sha1;
use sha2::Sha256;
use sha3::{Sha3_224, Sha3_256, Digest};
use hexx::{Hex16, Hex20, Hex28, Hex32};

pub struct Sums {
    pub md5: Hex16,
    pub sha1: Hex20,
    pub sha2_256: Hex32,
    pub sha3_224: Hex28,
    pub sha3_256: Hex32,
}

pub struct Summator {
    md5_context: md5::Context,
    sha1_context: Sha1,
    sha2_256_context: Sha256,
    sha3_224_context: Sha3_224,
    sha3_256_context: Sha3_256,
}

impl Default for Summator {
    fn default() -> Self {
        Self {
            md5_context: md5::Context::new(),
            sha1_context: Sha1::default(),
            sha2_256_context: Sha256::default(),
            sha3_224_context: Sha3_224::default(),
            sha3_256_context: Sha3_256::default(),
        }
    }
}

impl Write for Summator {
    #[inline]
    fn write(&mut self, data: &[u8]) -> IoResult<usize> {
        self.md5_context.consume(data);
        self.sha1_context.update(data);
        self.sha2_256_context.input(data);
        self.sha3_224_context.input(data);
        self.sha3_256_context.input(data);
        Ok(data.len())
    }

    #[inline]
    fn flush(&mut self) -> IoResult<()> {
        Ok(())
    }
}

impl Into<Sums> for Summator {
    fn into(self) -> Sums {
        Sums {
            md5: {
                Hex16(self.md5_context.compute().0)
            },
            sha1: {
                Hex20(self.sha1_context.digest().bytes())
            },
            sha2_256: {
                let mut a: [u8; 32] = [0; 32];
                a.copy_from_slice(self.sha2_256_context.result().as_slice());
                Hex32(a)
            },
            sha3_224: {
                let mut a: [u8; 28] = [0; 28];
                a.copy_from_slice(self.sha3_224_context.result().as_slice());
                Hex28(a)
            },
            sha3_256: {
                let mut a: [u8; 32] = [0; 32];
                a.copy_from_slice(self.sha3_256_context.result().as_slice());
                Hex32(a)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summator() {
        let mut summator = Summator::default();
        summator.write_all(b"Hello, ").unwrap();
        summator.write_all(b"World!").unwrap();
        let sums: Sums = summator.into();

        assert_eq!(sums.md5, "65a8e27d8879283831b664bd8b7f0ad4".parse::<Hex16>().unwrap());
        assert_eq!(sums.sha1, "0a0a9f2a6772942557ab5355d76af442f8f65e01".parse::<Hex20>().unwrap());
        assert_eq!(sums.sha2_256, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f".parse::<Hex32>().unwrap());
        assert_eq!(sums.sha3_224, "853048fb8b11462b6100385633c0cc8dcdc6e2b8e376c28102bc84f2".parse::<Hex28>().unwrap());
        assert_eq!(sums.sha3_256, "1af17a664e3fa8e419b8ba05c2a173169df76162a5a286e0c405b460d478f7ef".parse::<Hex32>().unwrap());
    }
}
