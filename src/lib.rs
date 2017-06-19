#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]

extern crate rlibc;
extern crate compiler_builtins;

#[no_mangle]
pub extern fn rust_main() {
     let test = (0..3).flat_map(|x| 0..x).zip(0..);
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}