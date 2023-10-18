# hsize
Convert file sizes in bytes to human-readable units

```sh
$ hsize 1000 1000000 5000000
1.00 KB
1.00 MB
5.00 MB

$ hsize -p 5 1048576 12345678
1.04858 MB
12.34568 MB

$ hsize 1048576 --binary
1.00 MiB

$ echo -e "10\n1000\n1000000000" | hsize
10.00 B
1.00 KB
1.00 GB
```

## Installation

### Nix flake
```shell
$ nix run github:ErrorNoInternet/hsize -- 1000 1000000 5000000
```

### cargo
```shell
$ git clone https://github.com/ErrorNoInternet/hsize
$ cd hsize
$ cargo install --path .
```
