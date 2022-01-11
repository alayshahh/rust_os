
#![no_std] // unlinking the rust std library
#![no_main]
/*
Once the std library is unlinked, there is no start atribute
What does this mean?
this means that the runtime system that is initalized before calling the main method is no  longer there since we removed the std library
we no longer have the proper set up do run the program
Typically, the OS will call the runtime which will then do the init set up then it will the main method from the program we wrote
Rust links the C standdard library with its own std lib, which then calls C runtime zero (crt0)
crt0 sets up the env for a C application
then this envokes the rust runtime, which sets a few more things up, then finally the main method is called
Since we dont want to use the std library, we will now need to create our own _start function to set up the runtime for the OS & thus we have no main method just yet
*/

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
	let vga_buffer = 0xb8000 as *mut u8;
	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe{
			*vga_buffer.offset(i as isize *2) = byte;
			*vga_buffer.offset(i as isize *2 +1) = 0xb;
		}
	}
    loop {}
}


use core::panic::PanicInfo;

//this function is called on panic

#[panic_handler] 
//our own panic handler, doesnt do much as of now, just loops infnitely
fn panic (_info: &PanicInfo) -> ! {
	loop {}
}



