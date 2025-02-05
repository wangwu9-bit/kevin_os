#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic_(_info:&PanicInfo)->!{
    loop {
        
    }
}

#[no_mangle]
pub extern "C" fn _start()->!{
    loop {
        
    }
}