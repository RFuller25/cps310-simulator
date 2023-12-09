# Journal

## Digest for Rough Draft
### Easiest parts 
Fetching from memory and storing the registers and flags.
### Hardest parts 
Decoding instructions efficiently, abstracting parts in logical manner.
### Estimated Time 
60+ hours  
### Experimentation 
I think generally my design logic of abstracting parts will be optimal, and it seemed to work well for implementing the one instruction. Test program passed, and creating debug messages along the way I found that the system worked exactly as I intended it to (after some debugging and fixing) which was encouraging.

The hardest part will be an efficient method of if/else / case statements to digest instructions, but I think with experience and further learning this will be more intuitive. 

## Rust
After Dr. Jueckstock's help in getting the Rust and C to talk to each other, I think I am at the place where I am able to accomplish the CPU in rust, or at least make a good college try at it. 

I bought the book "Rust Programming For Beginners" by Nathan Metzler on sale, and I have been consulting it heavily. In addition, I have been making heavy usage of the [doc.rust-lang.org](https://doc.rust-lang.org) (Rust Online Documentation.)

For specific problems that neither of those can help me with, I've been using stack overflow forums:
- https://stackoverflow.com/questions/56485167/how-to-format-a-byte-into-a-2-digit-hex-string-in-rust
- https://stackoverflow.com/questions/51571066/what-are-the-exact-semantics-of-rusts-shift-operators
- https://stackoverflow.com/questions/65261859/why-cant-i-index-a-u32-with-a-u32

I briefly consulted ChatGPT in an effort to get C and Rust to interface, and while it was able to help me with what I asked it for, I simply did not know enough about what I needed to do in order to ask the right questions. A transcript of the conversation and a brief commentary is available in the PDF file `chatgptrust.pdf`.




## Timeline

- Rough Design - **9/25**
    - +7 hours
    - Minimum working example
    - Focus on basics of design, namely, seperation of files
- Refined Design - **10/06**
    - +10-15 hours
    - Basics implemented of well-thought out implementation to digest commands
    - Tests built and passing
- Prototype - **10/18**
    - +10-15 hours
    - Using foundations already built, build it to pass all official tests.
    - Decide on EC options
- Simulator - **11/18**
    - +20 hours
    - All tests are passing.
    - EC options are completed and functional
    - Self-made test that shows of EC and full functionality is built and functioning well.

## Log
| Date | Description | Time Spent |
| ---- | ----------- | ---------- |
| 9/16 | Worked on implementing rust for 4 hours, realized that I was not going to get it done in time, so I implemented the mockup in C instead, completed it in 2 hours |  6 hours |
| 9/23 | Worked on Design, built journal | 1 hour |
| 9/24 | Finished design, set up Github, started working on rough draft | 4 hours |
| 9/25 | Polished design (apparently not finished), implemented one instruction | 3 hours |
| 9/30 | Worked on trying to get Rust to interface properly, realized it was a bit complicated | 2 hours |
| 10/02 | Consulted multiple websites trying to figure out how to get rust to send a char star star. Discovered the wonders of bindgen, and tried to get it to help me. | 2 hours |
| 10/03 | Tried to find alternative ways to passing it, including CString. Got a char * working, and was experimenting with muts and mut muts to try to get that to work | 2 hours |
| 10/04 | After hitting many brick walls, decided to consult ChatGPT and ask it to help me find a way around interfacing. | 2 hours |
| 10/05 | Worked with what ChatGPT gave me, tried to get it to work and coerce the code into getting something close, was unable to. | 2 hours |
| 10/06 | Tried to start the C version, realized that was not going to happen by deadline. | 1 hour | 
| 10/09 | Tried to start the C version, was too sick. Dr. J mentioned he would help with Rust. | 15 minutes |
| 10/10 | Got the Rust code, worked on it but had to prioritize other homework | 30 minutes |
| 10/12 | Read through the rust code to try to get to a point where I understood it well. Got the code up to the rough design portion. | 2 hours |
| 10/13 | Got the code to the point it is currently. Implemented the other MOV instructions and the LDR, went through the design instructions.  | 5 hours |