# Trident Test
Simple example program on Solana using the Trident fuzzer testing framework 

## Dependencies
- Anchor 0.30.0
- Solana
- Rust
- Trident

## Usage
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

## Explanation
### Bug
In `lib.rs`, we have `initialize` and `update` which stores a `u8` inside of `User`. `update` should require the `verify` parameter to be divisible by 4, but instead it has a "typo" and checks if it is divisible by 5. 

### Fuzzer
Now, looking at `fuzz_instructions.rs`, we are checking whether the call was valid through of the `check()` function, where we make sure that whatever `verify` passed in is divisible by 4 (which it will catch an error for). 

### Debugger
<img width="1054" alt="Screenshot 2024-06-12 at 12 58 13 PM" src="https://github.com/RohanVSuri/trident_test/assets/32184195/4fb5ee93-7e1a-47c1-9676-8d8a5b8b964e">

The `verify` that was passed in is divisible by 5, which was caught by the `check()` function (which is run after every call of `update`) and then threw the `DataMismatch` error since it's not divisible by 4. 

## Notes
- This cannot be run on macOS on Apple Silicon; I used a GitHub Codepsace to run it and would recommend the same
- Refer to Trident documentation [HERE](https://ackee.xyz/trident/docs/latest/)
