#![feature(lang_items, main)]
#![no_std]
#![no_main]


#[no_mangle]
#[allow(dead_code)]
#[allow(unreachable_code)]
pub extern "C" fn abort() {
    loop {}
}


#[no_mangle]
#[allow(dead_code)]
#[allow(unreachable_code)]
#[main]
pub extern "C" fn main() -> i32 {
    let mut a = 0i32;
    loop {
        a += 1;
    }
    return 0;
}


#[lang="eh_personality"]
#[allow(dead_code)]
#[allow(unreachable_code)]
extern fn eh_personality() {
    loop {}
}


#[lang="panic_fmt"]
#[allow(dead_code)]
#[allow(unreachable_code)]
extern fn panic_fmt() -> ! {
    loop {}
}

