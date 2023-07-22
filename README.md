# msr-explorer

msr-explorer is a dumb tool for Linux to explore the x86 MSR space.

## Requirements

* Linux:
    * `msr` kernel modules loaded
* Root privileges

## Usage

```shell
$ cargo build
$ # Read MSR 0x1b from CPU 0
$ sudo ./target/debug/msr-explorer -c 0 rdmsr 0x1b
$ # Read MSRs in range [0x0000-0x2000) from CPU 0
$ sudo ./target/debug/msr-explorer -c 0 rdmsr-range 0x0000 0x2000
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.