#![no_std]

use core::panic::PanicInfo;
fn main() {

}


#[panic_handler]
fn panic_(_info:&PanicInfo)->!{
    loop {
        
    }
}