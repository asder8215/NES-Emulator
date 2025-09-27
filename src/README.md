# NES-Emulator
An NES emulator following "Write NES Emulator in Rust" by Bugzmanov

# File Structure
```
├── src/
│   ├── cpu      # Contains 6502 instruction set, addressing mode, memory, etc.
│   ├── bus      # For intra device comms, mmapping, coord PPU & CPU cycles
│   └── rom      # Reads in ROM files
│   └── ppu      # Renders graphics and state of the screen
│   └── gamepad  # Parses input from game pad
│   └── apu      # Process and generate audio from game
```
Each folder contains a mod.rs file that represents the parent file for CPU/Bus/ROM/PPU/GamePad/APU code

