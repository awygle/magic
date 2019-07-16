use gumdrop::Options;
use std::error::Error;
use std::collections::VecDeque;
use arraydeque::{ArrayDeque, Saturating};
use ux::{u20, u14};

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
    //fn fetch(&mut self, addr: u32) -> [u8; 8];
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

#[derive(Clone, Copy)]
pub struct ICacheLine {
    data: [u32; 8],
    tag: u20,
    valid: bool,
}

impl Default for ICacheLine {
    fn default() -> ICacheLine {
        ICacheLine {
            data: [0; 8],
            tag: 0u16.into(),
            valid: false,
        }
    }
}

pub trait InstructionCache : Default {
    type AddressSize;
    type Output;
    fn fetch(&self, addr: Self::AddressSize) -> Self::Output;
}

pub struct ICache {
    memory: [ICacheLine; 512]
}

impl Default for ICache {
    fn default() -> ICache {
        ICache {
            memory: [ICacheLine::default(); 512],
        }
    }
}

impl InstructionCache for ICache {
    type AddressSize = u14;
    type Output = ICacheLine;
    fn fetch(&self, addr: Self::AddressSize) -> Self::Output {
        self.memory[u32::from(addr >> 5) as usize]
    }
}

enum CpuCommand {
    InstructionFetch,
    WaitMem(u8),
}

pub struct InterpCPU32bit<MB, IC, MM> 
    where MB: MemoryBus, 
    IC: InstructionCache,
    MM: MMU
{
    pc: u32,
    bus: MB,
    icache: IC,
    mmu: MM,
    cmd_queue: VecDeque<ArrayDeque<[CpuCommand; 8], Saturating>>
}

pub trait CPU {
    fn new(pifrom_src: &mut impl std::io::Read) -> Self;
    fn run(self);
}

const RESET_VECTOR_32: u32 = 0xBC00_0000;
const MEM_WORD_DELAY: usize = 38;

impl<MB: MemoryBus, IC: InstructionCache, MM: MMU> CPU for InterpCPU32bit<MB, IC, MM> 
where MM::AddressSize : From<u32>
{
    fn new(pifrom_src: &mut impl std::io::Read) -> Self {
        InterpCPU32bit {
            pc: RESET_VECTOR_32,
            bus: MB::new(pifrom_src),
            icache: IC::default(),
            mmu: MM::default(),
            cmd_queue: VecDeque::with_capacity(64),
        }
    }
    
    /*fn instruction_fetch(&mut self, _: CpuCommand) {
        if !self.mmu.is_cached(self.pc) {
            self.bus.fetch(self.pc);
            self.cmd_queue.push_back(CpuCommand::WaitMem(MEM_WORD_DELAY));
        }
    }*/
    
    fn run(mut self) {
        loop {
            while let Some(cmd) = self.cmd_queue.pop_front() {
                match cmd {
                    InstructionFetch => {
                        
                    }
                }
            }
        }
    }
}

impl<MB: MemoryBus, IC: InstructionCache, MM: MMU> InterpCPU32bit<MB, IC, MM> {
    fn icache_fetch(icache: &IC, addr: IC::AddressSize) -> IC::Output {
        icache.fetch(addr)
    }
}

pub trait MMU : Default {
    type AddressSize;
    fn is_cached(addr: Self::AddressSize) -> bool;
}

pub struct MMU32Bit {
    
}

impl Default for MMU32Bit {
    fn default() -> Self {
        MMU32Bit {}
    }
}

impl MMU for MMU32Bit {
    type AddressSize = u32;
    
    fn is_cached(addr: u32) -> bool {
        addr >= 0xA000_0000 && addr < 0xC000_0000
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
