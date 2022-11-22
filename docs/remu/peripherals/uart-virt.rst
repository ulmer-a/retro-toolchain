Virtual UART
------------

The virtual UART is a serial port designed to be as easy to interface with as
possible.

.. list-table:: Register Map
   :header-rows: 1

   * - Offset
     - READ
     - WRITE
   * - 0x00
     - RX Data Buffer
     - TX Data Buffer
   * - 0x01
     - Status Register
     - Control Register


Example
~~~~~~~

The following example prints *Hello, world!* to the serial port by repeatedly
writing the UART data register.

.. code-block:: asm

    .segment "code"
    
    VIRT_UART_DATA = 0x8000

    _start:
        ldx #$00
    L1: lda hello_world, x
        beq L2
        sta VIRT_UART_DATA
        inx
        jmp L1
    L2: wai
        jmp L2

    .segment "data"

    hello_world:
        .byte "Hello, world!", 0a, 00


Data Register
~~~~~~~~~~~~~~

The UART data register is located at offset 0x00 from the peripheral's
base address. Read from this register to receive the next byte from the
receive buffer. Write to this register to transmit a byte.

Both the receive and transmit buffers are guaranteed to hold **at least
256 bytes** before they overflow and potentially overwrite previous data.
A full transmit buffer is indicated by bit 2 (**F**) in the status register.

.. warning::

    Reading from the data register is undefined behaviour if the **A**-bit
    (bit 0) in the status register is zero.


Status Register
~~~~~~~~~~~~~~~

The UART status register is located at offset 0x0001 (read) from the
peripheral's base address and has the following layout:

.. code-block: txt

    [ Virt UART Status Register (Offset )]

      7                       1   0
    +-----------------------+---+---+
    |        Reserved       | F | A |
    +-----------------------+---+---+

* Bit 0 (**A**): *RX Data Available*. This bit is going to be set to 1 if and
only if there is data to be read from the receive buffer.
* Bit 1 (**F**): *TX Buffer Full*. This bit is going to be set to 1 if and only
if the transmit buffer is full. Writing to the data register when this is 1 may
overwrite previous data in the transmit buffer.
* Bit 2-7: *Reserved*.


Control Register
~~~~~~~~~~~~~~~~

The UART control register is located at offset 0x0001 (write) from the
peripheral's base address and has the following layout:

.. code-block: txt

    [ Virt UART Status Register (Offset )]

      7                       1    0
    +-----------------------+---+----+
    |        Reserved       | E | RI |
    +-----------------------+---+----+

* Bit 0 (**I**): *RX Interrupt Enable*. Set to 1 to generate an interrupt
whenever a data byte becomes available at the RX data register.
* Bit 1 (**E**): *Echo Mode Enable*. Setting this to 1 will feed every
transmitted character back into the receiver.
* Bit 2-7: *Reserved*.
