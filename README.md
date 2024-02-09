# hsize

Convert file sizes to and from human-readable units

```sh
$ hsize 1000 1000000 5000000
1.00 KB
1.00 MB
5.00 MB

$ hsize -p 5 1048576 12345678
1.04858 MB
12.34568 MB

$ hsize 1048576 1073741824 --binary
1.00 MiB
1.00 GiB
```

### Read from stdin

```sh
$ printf "10\n1000\n1000000000" | hsize -p 0
10 B
1 KB
1 GB

$ echo $RANDOM | hsize
26.07 KB

$ printf "5\n84\n" | hsize -f g -t m
5000.00 MB
84000.00 MB

$ echo "200  512  1" | hsize -f g -t b -B replace
214748364800.00 B  549755813888.00 B  1073741824.00 B
```

### Replace

Use regex to search and replace numbers

```sh
$ echo "1  22  333  4444  55555  666666  7777777  88888888  999999999" | hsize replace
1.00 B  22.00 B  333.00 B  4.44 KB  55.55 KB  666.67 KB  7.78 MB  88.89 MB  1000.00 MB

$ cat /proc/meminfo | sed "s| kB||; s| ||g" | hsize -f k replace -r "\d+$" | column -ts :
MemTotal         16.31 GB
MemFree          929.67 MB
MemAvailable     10.87 GB
Buffers          44.00 KB
Cached           10.71 GB
SwapCached       6.38 MB
Active           5.52 GB
Inactive         8.10 GB
Active(anon)     3.59 GB
Inactive(anon)   214.04 MB
Active(file)     1.93 GB
Inactive(file)   7.88 GB
Unevictable      329.73 MB
Mlocked          120.00 KB
SwapTotal        32.61 GB
[...]

$ stat flake.nix | hsize -b replace -r "Size: (\d+).*IO Block: (\d+)"
  File: flake.nix
  Size: 1.77 KiB      	Blocks: 8          IO Block: 4.00 KiB   regular file
Device: 0,68	Inode: 2522344     Links: 1
[...]
```

## Usage

### Binary

#### Nix

```shell
$ nix run github:ErrorNoInternet/hsize -- 1000 1000000 5000000
```

#### Cargo

```shell
$ cargo +nightly install --git https://github.com/ErrorNoInternet/hsize
```

### Library

```rust
use hsize::{format::Options, Converter, Scale, Unit};

fn main() {
    let converter = Converter {
        from_unit: Unit {
            // 1K = 1000
            is_binary: false,
            // `None` for byte (Scale::B)
            scale: Some(Scale::K),
        },
        to_unit: Unit {
            // 1K = 1024
            is_binary: true,
            // `None` for automatic scaling
            scale: None,
        },
    };

    // 1073741824 KB = 1000 GiB
    assert_eq!(converter.convert(1073741824), (1000.0, Scale::G));
    assert_eq!(converter.format(1073741824, 3), "1000.000 GiB");
    assert_eq!(
        converter.format_with_options(
            1073741824,
            &Options {
                precision: 0,
                separator: "",
                ..Options::default()
            }
        ),
        "1000GiB"
    );
}
```
