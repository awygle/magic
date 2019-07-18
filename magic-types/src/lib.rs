use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use ux::{u5, u6};

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum InstructionType {
    R,
    I,
    J,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Register {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    R31,
    RD,
    RT,
    RS,
    FT,
    FS,
    FD,
    CT,
    CS,
    CD,
    HI,
    LO,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Extend {
    #[serde(rename = "sext")]
    Sign,
    #[serde(rename = "zext")]
    Zero,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Exception {
    Overflow,
    Underflow,
    TlbMiss,
    TlbInvalid,
    BusError,
    AddressError,
    ReservedInstruction,
    TlbModification,
    CoprocessorUnusable,
    SystemCall,
    Breakpoint,
    IntegerOverflow,
    Trap,
    InvalidOperation,
    InexactOperation,
    DivisionByZero,
}

#[derive(Copy, Clone, Debug)]
pub struct Opcode(pub u6);

impl Serialize for Opcode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:06b}", self.0))
    }
}

struct OpcodeVisitor;

impl<'de> Visitor<'de> for OpcodeVisitor {
    type Value = Opcode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 64")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(x) = u8::from_str_radix(value, 2) {
            if x >= 64 {
                return Err(E::custom(format!("u6 out of range: {}", x)));
            }
            return Ok(Opcode(u6::new(x)));
        }
        Err(E::custom(format!(
            "str cannot be parsed as binary u6: {}",
            value
        )))
    }
}

impl<'de> Deserialize<'de> for Opcode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(OpcodeVisitor)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RegNum(pub u5);

impl Serialize for RegNum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{:05b}", self.0))
    }
}

struct RegNumVisitor;

impl<'de> Visitor<'de> for RegNumVisitor {
    type Value = RegNum;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 64")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Ok(x) = u8::from_str_radix(value, 2) {
            if x >= 64 {
                return Err(E::custom(format!("u5 out of range: {}", x)));
            }
            return Ok(RegNum(u5::new(x)));
        }
        Err(E::custom(format!(
            "str cannot be parsed as binary u5: {}",
            value
        )))
    }
}

impl<'de> Deserialize<'de> for RegNum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(RegNumVisitor)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct MetaInstruction {
    pub name: String,
    #[serde(rename = "type")]
    pub itype: InstructionType,
    #[serde(rename = "use")]
    pub use_regs: Vec<Register>,
    pub def: Vec<Register>,
    pub opcode: Opcode,
    pub branch: bool,
    pub exceptions: Vec<Exception>,
    pub extend: Extend,
    pub rt: Option<RegNum>,
    pub rs: Option<RegNum>,
    pub rd: Option<RegNum>,
    pub sa: Option<RegNum>,
    pub funct: Option<Opcode>,
}

impl Default for MetaInstruction {
    fn default() -> Self {
        MetaInstruction {
            name: "".to_string(),
            itype: InstructionType::I,
            use_regs: Vec::new(),
            def: Vec::new(),
            opcode: Opcode(u6::new(0)),
            branch: false,
            exceptions: Vec::new(),
            extend: Extend::Zero,
            rt: None,
            rs: None,
            rd: None,
            sa: None,
            funct: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serialize_itype() {
        let itype = InstructionType::I;
        assert_eq!(serde_json::to_string(&itype).unwrap(), r#""I""#);
    }

    #[test]
    fn deserialize_itype() {
        let rtype: InstructionType = serde_json::from_str(r#""R""#).unwrap();
        assert_eq!(rtype, InstructionType::R);
    }

    #[test]
    fn serialize_regs() {
        let reg = Register::R23;
        assert_eq!(serde_json::to_string(&reg).unwrap(), r#""r23""#);
    }

    #[test]
    fn deserialize_regs() {
        let reg: Register = serde_json::from_str(r#""r15""#).unwrap();
        assert_eq!(reg, Register::R15);
    }

    #[test]
    fn serialize_opcode() {
        let opcode = Opcode(u6::new(15));
        assert_eq!(serde_json::to_string(&opcode).unwrap(), r#""001111""#);
    }

    #[test]
    fn deserialize_opcode() {
        let opcode: Opcode = serde_json::from_str(r#""110110""#).unwrap();
        assert_eq!(opcode.0, u6::new(54));
    }

    #[test]
    fn serialize_regnum() {
        let regnum = RegNum(u5::new(15));
        assert_eq!(serde_json::to_string(&regnum).unwrap(), r#""01111""#);
    }

    #[test]
    fn deserialize_regnum() {
        let regnum: RegNum = serde_json::from_str(r#""10110""#).unwrap();
        assert_eq!(regnum.0, u5::new(22));
    }

    #[test]
    fn serialize_exception() {
        let except = Exception::Overflow;
        assert_eq!(serde_json::to_string(&except).unwrap(), r#""overflow""#);
    }

    #[test]
    fn deserialize_exception() {
        let except: Exception = serde_json::from_str(r#""overflow""#).unwrap();
        assert_eq!(except, Exception::Overflow);
    }
}
