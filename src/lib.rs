use gumdrop::Options;
use std::error::Error;

#[derive(Debug, Options)]
pub struct EmuOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "path to PIF boot ROM", long = "pifrom", required)]
    pub pifrom_path: String,
}

pub struct Memory {
    pifrom: [u8; 2048],
}

pub trait MemoryBus {
    fn new(pifrom_src: &mut impl std::io::Read) -> Self;
}

impl MemoryBus for Memory {
    fn new(pifrom_src: &mut impl std::io::Read) -> Self {
        let mut result = Memory { pifrom: [0; 2048] };
        if let Err(why) = pifrom_src.read(&mut result.pifrom) {
            panic!("Couldn't read pifrom: {}", why.description());
        }
        result
    }
}

pub struct InterpCPU32bit<MB: MemoryBus> {
    pc: u32,
    bus: MB,
}

pub trait CPU {
    fn new(pifrom_src: &mut impl std::io::Read) -> Self;
}

const RESET_VECTOR_32: u32 = 0xBC00_0000;

impl<MB: MemoryBus> CPU for InterpCPU32bit<MB> {
    fn new(pifrom_src: &mut impl std::io::Read) -> Self {
        InterpCPU32bit {
            pc: RESET_VECTOR_32,
            bus: MB::new(pifrom_src),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gumdrop::ParsingStyle;
    #[test]
    fn pifrom_present() {
        let args: &[&str] = &["--pifrom=pifdata.bin"];
        let opts = EmuOptions::parse_args(args, ParsingStyle::AllOptions).unwrap();
        assert_eq!(opts.pifrom_path, "pifdata.bin");
    }

    #[test]
    #[should_panic]
    fn pifrom_required() {
        let args: &[&str] = &[""];
        let opts = EmuOptions::parse_args(args, ParsingStyle::AllOptions).unwrap();
        assert_eq!(opts.pifrom_path, "pifdata.bin");
    }

    #[test]
    fn help_no_pifrom() {
        let args: &[&str] = &["--help"];
        let _opts = EmuOptions::parse_args(args, ParsingStyle::AllOptions).unwrap();
    }
}
