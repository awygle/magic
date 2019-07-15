use gumdrop::Options;
use magic::*;
use std::error::Error;
use std::fs::File;
use std::path::Path;

fn main() {
    let opts = EmuOptions::parse_args_default_or_exit();
    let pifrom_path = Path::new(&opts.pifrom_path);

    let mut file = match File::open(&pifrom_path) {
        Err(why) => panic!(
            "Couldn't open pifrom file {}: {}",
            pifrom_path.display(),
            why.description()
        ),
        Ok(file) => file,
    };

    let _cpu = InterpCPU32bit::<Memory, ICache, MMU32Bit>::new(&mut file);

    println!("{:#?}", opts);
}
