pub struct Hash {
    bytes: Vec<u8>
}

impl Hash {
    pub fn new() -> Hash {
        Hash {
            bytes: vec!(),
        }
    }

    pub fn from(bytes: Vec<u8>) -> Hash {
        Hash {
            bytes,
        }
    }
}

impl std::cmp::PartialEq for Hash {
    fn eq(self: &Self, s2: &Self) -> bool {
        self.bytes == s2.bytes
    }
}

impl std::fmt::Display for Hash {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in &self.bytes {
            if let Err(err) = write!(f, "{:02x}", byte) {
                return Err(err);
            }
        }
        Ok(())
    }
}

impl std::clone::Clone for Hash {
    fn clone(self: &Self) -> Self {
        return Hash {
            bytes: self.bytes.clone()
        }
    }
}
