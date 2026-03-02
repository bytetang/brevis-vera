# ELF Output Directory

This directory contains the compiled RISC-V ELF binary produced by `cargo pico build`.

The ELF file (`riscv32im-pico-zkvm-elf`) is NOT committed to version control.
Build it locally with:

```bash
cd zk-guest/app
cargo pico build
```

The host prover loads this ELF to execute the guest program inside the Pico ZKVM.
