#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use core::{fmt::Write, panic::PanicInfo};
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, mut st: SystemTable<Boot>) -> Status {
    // st.stdout().reset(false).unwrap();

    //st.stdout().write_str("Hello, world!").unwrap();
    writeln!(st.stdout(), "Hello, world!").unwrap();

    // loop {}
    Status::SUCCESS
}
