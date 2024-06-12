# Trident Test
Simple example program on Solana using the Trident fuzzer testing framework 

## Setup
1. Build the contracts
```bash
anchor build
```

2. Run the fuzzer until a crash (unnecessary, .fuzz file is already in repo) 
```bash
trident fuzz run fuzz_0
```

3. Run Debugger
```bash
trident fuzz run-debug fuzz_0 PATH-TO-FUZZ.fuzz

## example:
trident fuzz run-debug fuzz_0 trident-tests/fuzz_tests/fuzzing/hfuzz_workspace/fuzz_0/SIGABRT.PC.7ffff7c7100b.STACK.1bd25de105.CODE.-6.ADDR.0.INSTR.mov____0x108(%rsp),%rax.fuzz
```

## Notes
- This cannot be run on macOS on Apple Silicon; I used a GitHub Codepsace to run it and would recommend the same
