# hsize
Display file sizes in human-readable units

```sh
$ hsize 1000
1.00 KB

$ hsize -p 5 1048576
1.04858 MB

$ hsize 1048576 --binary
1.00 MiB

$ echo -e "10\n1000\n1000000000" | hsize
10.00 B
1.00 KB
1.00 GB
```

## Installation
```sh
git clone https://github.com/ErrorNoInternet/hsize
cd hsize
cargo install --path .
```

