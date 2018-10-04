use std::fs::File;

use crate::err::Result;

use goblin::elf;

use memmap::{Mmap, MmapOptions};

pub struct MappedElf<'a> {
    mmap: &'a Mmap,
    pub elf: elf::Elf<'a>,
}

impl<'a> MappedElf<'a> {
    pub fn from_map(mmap: &'a Mmap) -> Result<Self> {
        let elf = elf::Elf::parse(mmap)?;

        Ok(MappedElf { mmap, elf })
    }
}
