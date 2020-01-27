
use std::convert::TryInto;
use std::fs;
use std::io::ErrorKind::UnexpectedEof;
use std::io::Read;
use magic::*;

#[test]
fn parse_example_list() {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("shade_list.bin").expect("couldn't find input file");
    
    let num_bytes :usize = (file.metadata().expect("couldn't get file data").len()).try_into().unwrap();
    let mut command_bytes :Vec<u8> = Vec::with_capacity(num_bytes);
    command_bytes.resize_with(num_bytes, || 0);
    file.read(&mut command_bytes).expect("not enough commands");
    
    assert!(command_bytes.len() == num_bytes);
    let mut command_slice = &command_bytes[..];
    
    loop {
        let command_option: std::io::Result<RDPCommand> = command_slice.read_command();
        if let Ok(command) = command_option {
            println!("Got command {:#X?}", command);
        }
        else {
            println!("Unknown command or result {:?}", command_option);
            if command_option.unwrap_err().kind() == UnexpectedEof {
                break;
            }
        }
    }
}
