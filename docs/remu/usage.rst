----------------------------------
Usage/Invocation
----------------------------------

.. code-block:: txt

    remu {options}

Standard options
~~~~~~~~~~~~~~~~

.. code-block:: txt

    --board={<json-device-tree>|<built-in-board>}

The ``--board`` option selects the SBC target board to be emulated. Either
set this to a built-in board identifier or specify a :doc:`JSON file describing
your own SBC <device_tree_format>`. Known built-in boards are:

* ``virt6502`` Virtual Target with NMOS 6502, 48K RAM, 16K ROM, UART.
* ``virt65c02`` Virtual Target with WDC 65C02, 48K RAM, 16K ROM, UART.
* ``ulmer1`` Ulmer SBC Board with 65c02, 32K RAM, 8K ROM, ACIA and VIA.

.. code-block:: txt

    --cpu={6502|65c02}

Override the microprocessor used in the board currently selected by ``--board``.
