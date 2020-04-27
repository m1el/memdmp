# memdmp - dump process memory

After seeing a process snapshotting tool called `sausage_factory`,
I decided to explore an alternative in Rust.

The main idea is to create a relocatable code that doesn't depend on system libraries.
To do that, we use raw syscalls (listed and implemented in `src/syscalls.rs`).

Walking over memory regions is performed in `src/memory.rs`
by calling `NtQueryVirtualMemory` until it returns failure.

This tool creates a dump memory file created in `c:\users\null\memory_dump.bin` (hardcoded in `src/lib.rs`).

To check if it works, run `cargo run --release --example test`.

# (Un)Stability

The Windows syscall API is explecitly unstable, so this program is going to be incompatible across Windows versions.

# License

Licensed under MIT license, contains parts of `chocolate_milk` shared libraries.

Please see [LICENSE](LICENSE).
