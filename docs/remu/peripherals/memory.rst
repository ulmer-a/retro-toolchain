RAM/ROM Memory
----------------------------------

This peripheral provides generic memory that can optionally be read-only (rom).
All memory is initially initialized to zero.

.. code-block:: json

    {
        "type": "{ram|rom}",
        "name": "<name>",
        "addr": "<addr>",
        "size": "<size>",
        "file": "<path>"
    }

Reference
~~~~~~~~~

* ``type``: Either *ram* or *rom*. Behaviour is the same, except that *rom*
  memory is read-only. Required.
* ``name``: The memory peripheral can optionally be named so that it can be
  referenced on the command line (e.g. to initialize it).
* ``addr``: Address where the memory will be mapped. Required.
* ``size``: Size of the memory region in bytes. Required.
* ``file``: Specify a file path relative to the JSON file which will be
  used to initialize the memory. If the file is shorter than the memory region
  then the remaining memory will be initialized with zero. Optional.
