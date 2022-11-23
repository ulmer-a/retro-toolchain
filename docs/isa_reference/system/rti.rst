**RTI**: Return from Interrupt
---------------------------------------

The ``RTI`` instruction returns from an interrupt context by restoring the
status register and program counter from values on the stack.

.. list-table:: Opcodes
   :header-rows: 1

   * - Instruction
     - Opcode
     - Cycles
     - Affected Flags
   * - ``RTI``
     - 0x40
     - 6
     - **N**, **V**, **D**, **I**, **Z**, **C** [1]_


Interrupt Stack Layout
~~~~~~~~~~~~~~~~~~~~~~

The RTI instruction expects values to be present on the stack in the following
order:

.. code-block:: txt

   |     ....     |
   +--------------+
   |   PC (high)  |
   +--------------+  <= SP after executing `RTI`
   |   PC (low)   |
   +--------------+
   |      P       |
   +--------------+
   |              |
   +..............+  <= SP before executing `RTI`

.. warning::
   Note that when returning from a breakpoint (caused by an execution of the
   :doc:`BRK <brk>` instruction) the byte after the BRK instruction will be
   skipped. The reason for that is that the BRK instruction increments the
   program counter twice before storing it on the stack.

.. [1] Flags will be set based on the value found on the stack.
