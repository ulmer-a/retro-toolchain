----------------------------------
REMU SBC Emulator
----------------------------------

The **Retro Emulator** (REMU) is a highly configurable SBC emulator. It supports
both the original NMOS 6502 and the WDC 65C02 microprocessors and some peripheral
components.

All you have to do is describe your SBC in a simple device-tree file in JSON
format and start the emulator.

Eventually, I want to implement a fully-fledged debugger implementing the
Debug Adapter Protocol (DAP) and a wide variety of peripherals that you can
attach to your SBC. This serves as an ideal development platform.


.. toctree::
   :maxdepth: 1

   installation
   usage
   device_tree_format
   peripherals/index
