use std::fmt;
use std::hash::Hash;

#[derive(PartialEq, Eq, Hash)]
pub struct BlockId {
    filename: String,
    blknum: usize,
}

impl BlockId {
    pub fn new(filename: String, blknum: usize) -> BlockId {
        BlockId {
            filename: filename,
            blknum: blknum,
        }
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn number(&self) -> usize {
        self.blknum
    }
}

impl fmt::Display for BlockId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[file {}, block {}]", self.filename, self.blknum)
    }
}
