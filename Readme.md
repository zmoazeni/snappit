Example of statically compiling an external C library.

To run:

    brew install snappy
    cargo build -v # verify the additional compiler commands
    otool -L target/snappit # verify that snappy isn't dynamically linked
