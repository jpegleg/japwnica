# Japwnica

This is a demonstration of inserting of a binary (ELF) into a C program which loads the original ELF from text into RAM and then executes it from RAM.

```
$ bash ./japwnica /usr/bin/dmesg dmesgp
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/c_byte_array /usr/bin/dmesg`
$ ./dmesgp #dmesg code is loaded into RAM and executed from RAM
```

This tool is for educational purposes. Enjoy!

Requirements:
- gcc ( can be replaced by clang etc, just edit japwnica and swap gcc for the compiler of your choice)
- cargo ( the conversion tool c_byte_array is written in Rust, by default japwnica uses `cargo run` from pwd to execute it, however that can be replaced by a path to the compiled binary )
- sed
  
Compiling c_byte_array tool ahead of time example:

```
$ cargo build --release
   Compiling c_byte_array v0.1.0 (/home/admin/workspace/japwnicka)
    Finished release [optimized] target(s) in 0.21s
$ cp target/release/c_byte_array ./cba
$ sed -i 's/cargo run/.\/cba/g' japwnica
```
