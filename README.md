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
$ echo -e "10\n1000\n1000000000" | hsize
10.00 B
1.00 KB
1.00 GB

$ echo $RANDOM | hsize
26.07 KB

$ echo $RANDOM | hsize
11.72 KB

$ echo 12345678 | hsize -p3
12.346 MB
```

### Replace

Uses regex to search and replace numbers

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
```

## Installation

### Nix
```shell
$ nix run github:ErrorNoInternet/hsize -- 1000 1000000 5000000
```

### cargo
```shell
$ git clone https://github.com/ErrorNoInternet/hsize
$ cd hsize
$ cargo install --path .
```
