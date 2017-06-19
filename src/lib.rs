#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(unique)]
#![no_std]

extern crate volatile;
extern crate rlibc;
extern crate compiler_builtins;
extern crate spin;

#[macro_use] 
mod vga_buffer;

use vga_buffer::Writer;

#[no_mangle]
pub extern fn rust_main() {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");
    println!("{}", { println!("inner"); "outer" });

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}