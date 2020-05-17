# `longan-nano-examples`

> Example of using the longan-nano crate as an external library.
> 
> Uses latest version of from git https://github.com/riscv-rust/longan-nano as of 2020/05/17
>
> All examples are pulled directly from that crate, just combined into one main.rs
>
> Also has J-Link GDB support, though the default is left as per upstream
>
> To use this support, update the .cargo/config file and run JLink GDB server before cargo run:
>
> JLinkGDBServer -device GD32VF103CBT6 -speed 1000