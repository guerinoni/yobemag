use std::env;

mod cartridge;
mod cartridge_header;
mod cpu;
mod emulator;
mod memory_device;
mod opcodes;
mod prefix_opcodes;
mod register;

fn main() -> Result<(), std::io::Error> {
    println!("starting yobemag...");

    let args = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "missing rom as first arg.",
        ));
    }

    let rom = &args[1];
    println!("load of {}", &rom);

    let mut emu = emulator::Emulator::new(rom)?;

    loop {
        emu.step();
    }

    Ok(())
}
