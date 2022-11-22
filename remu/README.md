# REMU

6502, 65C02 and 65C816 Emulator inspired by QEMU.

## Workflow

1. Describe your custom SBC board and it's peripherals in a small JSON file
or use a predefined target.
2. Run `remu` and start executing your 6502/65C02/65C816 code.
3. Optionally, you can also attach the VSCode debugger frontend or any other
editor one supports the Debug Adapter Protocol. That way, you can single step,
peek and poke memory locations, etc.
