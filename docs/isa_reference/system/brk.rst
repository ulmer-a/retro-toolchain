**BRK**: Breakpoint
---------------------------------------

The ``BRK`` instruction triggers a software interrupt by first pushing the
instruction pointer and status register onto the stack and then jumping
to the address stored in the IRQ/BRK vector. The :doc:`RTI <rti>`
instruction can be used to return from the interrupt.

.. list-table:: Opcodes
   :header-rows: 1

   * - Instruction
     - Opcode
     - Cycles
     - Affected Flags
   * - BRK
     - 0x00
     - 7
     - **I = 1**, **D = 0** [1]_

The BRK vector is located at 0xfffe-0xffff.

Usage
~~~~~

The BRK instruction can be used to trigger a system calls to a kernel or
within a debugger to implement breakpoints.

Some systems also use it to raise an error. In that case the byte immediately
following the BRK instruction can be used to report an error number, since it
is usually skipped (see below).

Execution semantics
~~~~~~~~~~~~~~~~~~~

The BRK instruction will set the interrupt flag in the processor status register
to to one, disabling any further interrupts (except non-maskable interrupts which
will always be executed).

.. warning::
   The instruction pointer pushed onto the stack by the BRK instruction is
   actually incremented twice thus not pointing to the first but to the second
   byte after the BRK instruction itself.

   This means that the byte immediately following the BRK instruction **will be
   skipped** when the :doc:`RTI <rti>` instruction is used to return from the
   BRK condition.

   This can be used for debugging purposes or to pass along a single byte of data
   to the interrupt handling routine, for example, to encode a system call number.

Interrupt Stack Layout
~~~~~~~~~~~~~~~~~~~~~~~~~~

After the BRK instruction has been executed, the stack contains the program
counter (in little endian order) and status register.

.. code-block:: txt

   |     ....     |
   +--------------+
   |   PC (high)  |
   +--------------+  <= SP before executing `BRK`
   |   PC (low)   |
   +--------------+
   |      P       |     => in this value, the BRK flag is going to be `1`
   +--------------+
   |              |
   +..............+  <= SP after executing `BRK`

.. note::
   The BRK flag (bit 4 of the processor status register) is going to be set
   to one when the status register is pushed onto the stack after executing
   the BRK instruction. Since the maskable interrupt and the BRK interrupt
   share the same vector, this bit can be used by the interrupt handler to
   distinguish between an interrupt and a BRK condition.

   Note that the BRK bit in the processor status register will still always
   read back as zero. The bit will only ever be one in the value pushed onto
   the stack.

.. [1] The decimal flag is only cleared to zero on the WDC 65C02, but not on the
   original NMOS 6502.
