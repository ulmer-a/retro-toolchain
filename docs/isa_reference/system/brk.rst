**BRK**: Software interrupt
---------------------------------------

Opcode: `0x00`

   The ``BRK`` instruction triggers a software interrupt by first pushing the
   instruction pointer and status register onto the stack and then jumping
   to the address stored in the IRQ/BRK vector. The :doc:`RTI <rti>`
   instruction can be used to return from the interrupt.

The BRK vector is located at 0xfffe-0xffff.

Stack Layout
~~~~~~~~~~~~

After the BRK instruction has been executed, the stack contains the program
counter and status registers:

.. code-block:: txt

   |     ....     |
   +--------------+
   |   PC (high)  |
   +--------------+  <= SP before executing `BRK`
   |   PC (low)   |
   +--------------+
   |      P       |
   +--------------+
   |              |
   +..............+  <= SP after executing `BRK`


.. note::
   The instruction pointer pushed by the BRK instruction is actually incremented
   twice.

   TODO: Elaborate further and verify this on actual 6502 and 65c02 chips.
