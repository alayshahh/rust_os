
#![no_std] // unlinking the rust std library
/*
Once the std library is unlinked, there is no start atribute
What does this mean?
this means that the runtime system that is initalized before calling the main method is no  longer there since we removed the std library
we no longer have the proper set up to do run the program
Typically, the OS will 
*/
#![no_main]


use core::panic::PanicInfo;

//this function is called on panic

#[panic_handler] 
//our own panic handler, doesnt do much as of now, just loops infnitely
fn panic (_info: &PanicInfo) -> ! {
	loop {}
}



