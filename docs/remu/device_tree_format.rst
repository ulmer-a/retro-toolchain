----------------------------------
JSON Device Tree Format
----------------------------------

The JSON device tree describes an SBC board so that it can be emulated by REMU. 

Example
~~~~~~~

The following example device tree descrbes a board that has 32K of RAM, 8K
of ROM and a few peripherals mapped into it's address space.

With the ``uart-virt`` device you don't have to implement hardly any driver
code. Just write to it's memory location to print a character.

.. code-block:: json

    {
        "name": "My Custom 65c02 Board",
        "vendor": "John Doe",
        "description": "My Cool Board with 32k RAM and 8k ROM",
        "cpu": "65c02",
        "memory": [
            {
                "type": "ram",
                "addr": "0x0000",
                "size": "0x8000"
            },
            {
                "type": "rom",
                "addr": "0xE000",
                "size": "8192"
            }
        ],
        "peripherals": [
            {
                "type": "uart-virt",
                "addr": "0xdfd0"
            },
            {
                "type": "uart-65c51",
                "addr": "0xdfe0"
            },
            {
                "type": "via-65c22",
                "addr": "0xdff0"
            }
        ]
    }
