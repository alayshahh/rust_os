# VGA Text Mode Blog Post 3
VGA text mode is the simplest way to print text to the screen.

### What is VGA Text mode?
VGA (Video Grpahics Array) text mode is a standard intro'd by IBM. Main features are colored characters/background, blinking, different shaped cursor, loadable fonts

Traditionally, Linux console uses VGA text modes. Each screen character is 2 algned bytes, as 16 bits
1 bit for blinking, 3 bit background color, 3 bit foreground color, and 8 bit character. 

---

To print to a character to the screen in VGA mode, you have to write the character in the buffer of the VGA hardware, the buffer is a 2D array, typically 25 by 80, which is directly rendered on screen. Each array entry is 2 bytes.

- Byte 1 -> character in ASCII
- Byte 2 -> how the character is displayed
  
The VGA buffer is accessible to the CPU via a memory mapping to `0xb8000`. This means the reads and writes dont acess the RAm,but directly the text buffer on the VGA hardware. 

## The Implementation

We will create a Rust mdoule to handle the printing by using what we know abut VGA buffer.

### Colors
We will use an enum to represent the colors possible for the VGA buffer. We use the `=` syntax and the `repr(u8)` to force the enums to be of `u8` data type. We supress the unused enums warnings using the `allow(dead_code)` tag. we `derive` the `Copy`, `Clone`, `Debug`, `PartialEq`, and `Eq` traits to make the enum copyable, comparable and printable. 

We implement the `new` function to create the byte storing the foreground and background color in one byte. We do this by bit shifting the background color by 4 to the left and adding the foreground color.
### Text Buffer
The `repr(C)` tag is used to keep the struct ordering as defined in memory. The `repr(transparent)` is essentially the same thing but for a single feild struct.

The `ScreenChar` struct represents one array item in the VGA bffer array. The `Buffer` struct represents the actual VGA buffer.

#### The `Writer` struct:
 By default the writer will alaways write to the last line and shift up when a line is full or we encounter a new line character. The column position will keep track of where on that line the write should be. The `color_code` specifies the current foreground and background colors. The `&'static mut Buffer` is a reference to the VGA buffer. This reference needs an exlplicit lifetime to tell the compiler how long the reference to the VGA buffer is valid. In our case we wnat the reference to be valid for the entire runtime, so we use `'static`.

To print we use the Writer struct to modify the buffer's contents. 

## Volatility
We never actually red from the buffer we created, that is because it is the VGA buffer. It isnt something on the RAM that we use only for the lifetime of the program. Instead it is an actual location in the system memory. The compiler doesn't know that we do actually use it, and this could be a issue in future optimizations as the comiler may think that this value is never used. To safe guard against that we need to make these writes to memory volatile, i.e. tell the comipler to not optimize these operations as they have some side-effect. 

The volatile crate has a `Volatile` wrapper and `read` & `write` functions to tell the compiler to not optimize away these operations. 

## Formatting Macros 
To support Rust's built in formatting macro is to implement the `core::fmt::Write` trait for the `Writer` struct. We implement the `write_str` method for this.

## **Creating a Global Interface for the Writer**
### Attempt: Create a static WRITER 
The issue with this is that since static variables are init'd at compile time we cannot have a raw pointer converted to a reference at compile time. So we cannot point to the VGA buffer with a static WRITER.
## Solution: Lazy Statics
The `lazy_static` crate is our solution! It will init the static variable "lazily", i.e the varaible is not initilized until it is first used.  But...
### Issue
This writer is useless since it is immutable and our methods use `&mut self`. We could use a mutable static but then, we would run into data races abd undefined behaviors. Usually to get synchronization, we use Mutex locks, but those are deifned in the std library (which we dont have). 
Spinlocks are our answer. It is a basic mutex that doesnt require any OS features. Instead of blocking, the thread just tries locking over and over until it gets the mutex. We can use the `spin` crate for this.

## **Creating the println macro**
With a global writer, we can now add a `println` macro that can be used from anywhere. The code is based off the existing println macro from the standard library to make our lives easier.

Now with print functionality we can use the info from our panic function!

