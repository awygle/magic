#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use magic_types::*;
    use std::fs::File;
    use std::io::prelude::*;
    use ux::{u5, u6};

    pub fn instructions_iter(instr_db: &str) -> impl Iterator<Item = u32> {
        let mut f = File::open(instr_db).unwrap();
        let mut src = String::new();
        f.read_to_string(&mut src).unwrap();
        let instrs: Vec<MetaInstruction> = serde_json::from_str(&src).unwrap();
        let mut results: Vec<Box<dyn Iterator<Item = u32>>> = Vec::new();
        let mut i = 0;
        for instr in instrs {
            results.push(instr.legal_encodings());
            i = i + 1;
        }

        results.into_iter().flatten()
    }

    #[test]
    fn test_instruction_iterator() {
        let mut set: Vec<u8> = Vec::with_capacity(0x2000_0000);
        set.resize(0x2000_0000, 0);
        let mut count: u32 = 0;
        for instr in instructions_iter("../magic-macros/mipsiii.json") {
            let index: usize = instr as usize / 8;
            let mask = 1 << (instr % 8);
            assert_eq!(set[index] & mask, 0);
            set[index] |= mask;
            count += 1;
        }
        assert_eq!(count, 2_938_789_030);
    }
}
