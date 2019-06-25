use gumdrop::Options;

#[derive(Debug, Options)]
pub struct EmuOptions {
    #[options(help = "print help message")]
    help: bool,
    #[options(help = "path to PIF boot ROM", long = "pifrom", required)]
    pifrom_path: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use gumdrop::{Parser, ParsingStyle,Opt};
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
        let opts = EmuOptions::parse_args(args, ParsingStyle::AllOptions).unwrap();
    }
}
