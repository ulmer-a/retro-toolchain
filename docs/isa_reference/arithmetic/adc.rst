**ADC**: Add with carry
-----------------------

The ``ADC`` instruction adds an 8-bit value from memory to the accumulator
with carry. To perform an addition without carry, the carry flag must be
cleared first (for example with the :doc:`CLC <../system/sec_clc>` instruction).

.. list-table:: Opcodes
   :header-rows: 1

   * - Instruction
     - Opcode
     - Cycles
     - Affected Flags
   * - ``ADC #imm``
     - 0x69
     - 2
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC zp``
     - 0x65
     - 3
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC zp,x``
     - 0x75
     - 4
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC abs``
     - 0x6D
     - 4
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC abs,x``
     - 0x7D
     - 4 [2]_
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC abs,y``
     - 0x79
     - 4 [2]_
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC (abs, x)``
     - 0x61
     - 6
     - **N**, **V**, *(D)* [1]_, **Z**, **C**
   * - ``ADC (abs),y``
     - 0x71
     - 5 [2]_
     - **N**, **V**, *(D)* [1]_, **Z**, **C**

Usage
~~~~~

TODO

BCD Mode
~~~~~~~~

TODO

.. [1] The value computed by ADC depends on the decimal flag (see the section
   on BCD mode)
.. [2] +1 cycle if a page boundary is crossed while accessing two-byte memory
   locations.