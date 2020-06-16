# TODO-List for the chip8-oxidized emulator

- [x] Emulator
    - [x] Implement remaining instructions
    - [x] Implement font reading
    - [x] Figure out what to do with the 0NNN instruction
    - NOT NEEDED. Write test routines
    - [x] Implement interactive debugger
    - [x] SUGGESTION. Implement Super Chip8 (SCHIP8)
- [x] SDL frontend
    - [x] Handle time
    - [x] Implement efficient drawing
    - [x] Implement audio
    - [x] Perhaps implement different keybindings for non-numpad users
- [ ] WebAssembly Frontend
    - [ ] Implement frontend similar to the SDL one
- [x] Portability
    - [x] Separate emulator and rendering components
    - [x] SUGGESTION. Separate emulator loop and emulated architecture

NO WebAssembly Implementation
- No availability of regular arrays, only by using several libraries
- Complex structure code (due to the wasm_bindgen macro insertion)
