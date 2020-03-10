# WebAssembly for alt:V

A WebAssembly module for alt:V, written in Rust, utilizing the Wasmer Runtime.  

The module has memory virtualization implemented with a custom memory allocator
including a custom memory allocation algorithm implemented. This enables the use
malloc and other memory allocation functions available in native languages that
compile to WASM such as C, C++, Rust etc...

Please note that the memory allocator in the runtime was implemented by free-hand
and does not follow some well-known and published algorithm and may very well
contain bugs or inefficiencies. However compatability with existing native
languages is guaranteed and some efficacies are in place such as coalescing.  

Parallelism, concurrency and multithreading is not really features present for
WASM programs using this module. Considering that alt:V promotes an event-based
model for processing, it is not a priority for this module. Several languages
already have their own methods and implementations for how they handle concurrency
and paralellism.

If you believe this module is missing a fundemental feature that is required for
your language of choice (provided that the language compiles to WASM), feel free
to make an issue about it.