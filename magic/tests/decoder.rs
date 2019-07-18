#[cfg(test)]
mod tests {
    use magic_types::*;
    use std::fs::File;
    use std::io::prelude::*;
    use itertools::Itertools;
    use ux::{u5, u6};

    pub fn instructions_iter(instr_db: &str) -> impl Iterator<Item=u32> {
        let mut f = File::open(instr_db).unwrap();
        let mut src = String::new();
        f.read_to_string(&mut src).unwrap();
        let instrs :Vec<MetaInstruction> = serde_json::from_str(&src).unwrap();
        let mut results :Vec<Box<dyn Iterator<Item = u32>>>= Vec::new();
        let mut i = 0;
        for instr in instrs {
            let opcode: u32 = u32::from(instr.opcode.0) << 26;
            let mut local_results :Box<dyn Iterator<Item = u32>> = Box::new(opcode..=opcode);
            match instr.itype {
                InstructionType::R => {
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(funct) = instr.funct.map(|x| u32::from(x.0)) {
                            (funct..=funct)
                        } else {
                                (0..=u6::max_value().into())
                        }).map(|(x, y)| x | y));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(sa) = instr.sa.map(|x| u32::from(x.0)) {
                            (sa..=sa)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 6)));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(rd) = instr.rd.map(|x| u32::from(x.0)) {
                            (rd..=rd)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 11)));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(rt) = instr.rt.map(|x| u32::from(x.0)) {
                            (rt..=rt)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 16)));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(rs) = instr.rs.map(|x| u32::from(x.0)) {
                            (rs..=rs)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 21)));
                },
                InstructionType::I => {
                    local_results = Box::new(local_results
                        .cartesian_product(0..u16::max_value().into())
                        .map(|(x, y)| x | y));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(rt) = instr.rt.map(|x| u32::from(x.0) << 16) {
                            (rt..=rt)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 16)));
                    local_results = Box::new(local_results.cartesian_product(
                        if let Some(rs) = instr.rs.map(|x| u32::from(x.0)) {
                            (rs..=rs)
                        } else {
                            (0..=u5::max_value().into())
                        }).map(|(x, y)| x | (y << 16)));
                },
                InstructionType::J => {
                    local_results = Box::new(local_results
                        .cartesian_product(0..0x03FF_FFFF)
                        .map(|(x, y)| x | y));
                }
            }
            results.push(local_results);
            i = i + 1;
        }
        
        results.into_iter().flatten()
    }
    
    #[test]
    fn test_decoder() {
        for instr in instructions_iter("../magic-macros/mipsiii.json") {
            
        }
    }
}
