use std::fs::File;

use crate::err::{Error, Result};

use goblin::elf;

use memmap::Mmap;

/// Open the file with the given name and map it into memory.
///
/// Note that closing the file afterwards is not an issue, as the mapped region remains valid.
fn map_file(fname: &str) -> Result<Mmap> {
    let f = File::open(fname)?;
    let m = unsafe { Mmap::map(&f) }?;

    Ok(m)
}

/// A mapped ELF file we actually can process.
pub struct MappedElf<'a> {
    mmap: &'a Mmap,
    /// The wrapped ELF description provided by `goblin`.
    pub elf: elf::Elf<'a>,
}

impl<'a> MappedElf<'a> {
    /// Parse an ELF file from a memory mapping and validate it.
    pub fn from_map(mmap: &'a Mmap) -> Result<Self> {
        let elf = elf::Elf::parse(mmap)?;

        if MappedElf::check_support(&elf) {
            Ok(MappedElf { mmap, elf })
        } else {
            Err(Error::UnsupportedPlatform)
        }
    }

    fn check_support(elf: &elf::Elf) -> bool {
        // check that we deal with x86_64
        if elf.header.e_type != elf::header::ELFCLASS64.into() ||
                elf.header.e_machine != elf::header::EM_386 {
            return false;
        }

        // check that we have some program headers and section headers
        if elf.header.e_phnum == 0 || elf.header.e_shnum == 0 {
            return false;
        }

        true
    }
}
