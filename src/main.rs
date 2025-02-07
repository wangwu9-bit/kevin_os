#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info:&PanicInfo)->!{
    loop {
        
    }
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vbg_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vbg_buffer.offset(i as isize * 2) = byte;
            *vbg_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    unsafe {
        *vbg_buffer.offset(0) = b'M';    // 写入字符
        *vbg_buffer.offset(1) = 0x0F;    // 设置为白色
    }

    loop {}
}