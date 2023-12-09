# Design

## Types
Registers and the CSPR will be held in "word" sized memories. This is defined by the type header `typedef uint32_t	word;` as defined in `askapi.h`.

For statistics tracked by the CPU, instructions, loads, misses, load_misses, and store_misses, they will all be stored in the `RArmSimKernel` class, which holds all of the state for the CPU.

The configuration flags will be determined by the structure provided in `askapi.h`:

## Functions

### Decode - `rask_action.rs`

Decode takes in the state of registers and a single word instruction. It decodes the word, and then passes the state of the registers and the necessary parameters from the instruction to the respective function to perform the action.

### 

### ASKAPI functions - `ask.c`
All of the ASKAPI functions, will be implemented in `ask.c` in order to be public facing for the library.

### RASKAPI functions - `bindings.h`
These are functions to interface between the Rust and the C files.

### Action Functions
Functions will be implemented to perform the action for instructions. For example, an `data_processor_decode` function will take the parameters necessary and the state of the registers, perform the appropriate actions based on the operands, and then return the state of the registers and flags. These functions will be implemented with as much reusability in mind so as to minimize code size and the amount of bugs present. 

## Files
### ask.c
This file will act as the interface between the kernel and the CPU. It will hold all of the functions that are called as a part of library, and all functions publically available to be called. It holds all of the state via a global reference to the kernel object, and interfaces into the `lib.rs` file, which does all of the logic.

### lib.rs
This file acts as the go-between for the C and the Rust in the system. It contains all of the functions necessary for the C to ask the Rust to process instructions and provide updates, and then makes calls into `rask_action.rs`, passing appropriate state, in order to accomplish the actual work.

### rask_action.c
This file will act as the muscle of the system. There will be a function to decode the instruction, and if necessary that function will call necessary functions to complete actions such as loading memory and storing memory. Helper functions will be used to minimize code duplication and keep debugging simple.