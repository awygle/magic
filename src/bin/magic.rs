use magic::EmuOptions;
use gumdrop::Options;

fn main() {
    let opts = EmuOptions::parse_args_default_or_exit();
    
    println!("{:#?}", opts);
}
