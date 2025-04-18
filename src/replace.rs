use regex::Regex;

pub fn replace<'a>(
    input: &'a mut (impl Iterator<Item = String> + ?Sized),
    number_regex: &'a Regex,
    formatter: &'a dyn Fn(u128) -> String,
    left_align: bool,
) -> impl Iterator<Item = String> + 'a {
    input.map(move |line| {
        let mut new_line = line.clone();
        number_regex
            .captures_iter(&line)
            .map(|number_capture| {
                number_capture
                    .iter()
                    .collect::<Vec<_>>()
                    .iter()
                    .flat_map(ToOwned::to_owned)
                    .rev()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
            .for_each(|number_capture| {
                for number_match in number_capture {
                    if let Ok(number) = number_match.as_str().parse::<u128>() {
                        let formatted_number = &formatter(number);
                        if left_align {
                            new_line.replace_range(number_match.range(), formatted_number);
                        } else {
                            let padding = number_match
                                .range()
                                .len()
                                .saturating_sub(formatted_number.len());
                            new_line.replace_range(
                                number_match.range(),
                                &(" ".repeat(padding) + formatted_number),
                            );
                        }
                    }
                }
            });
        new_line
    })
}

#[cfg(test)]
mod tests {
    use super::replace;
    use hsize::{format::Options, Converter, Scale, Unit};
    use regex::Regex;

    fn owned_lines(string: &str) -> impl Iterator<Item = String> + '_ {
        string.lines().map(ToOwned::to_owned)
    }

    #[test]
    fn single() {
        let expected = owned_lines("19.028 PB");
        let mut input = owned_lines("19028310077231230");

        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        let regex = Regex::new(r"\d+").unwrap();
        let format = |size: u128| converter.format(size, 3);
        let output = replace(&mut input, &regex, &format, true);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }

    #[test]
    fn multiple() {
        let expected = owned_lines("12.06 KiB     1.00 B 1.00 B 1.00 B-------------1.00 MiB");
        let mut input = owned_lines("12345     1 1 1-------------1048576");

        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };
        let regex = Regex::new(r"\d+").unwrap();
        let format = |size: u128| converter.format(size, 2);
        let output = replace(&mut input, &regex, &format, true);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }

    #[test]
    fn stat() {
        let expected = owned_lines(
            "
              File: flake.nix
              Size: 1.81 KB      	Blocks: 8          IO Block: 4.10 KB   regular file
            Device: 0,68	Inode: 2522344     Links: 1
            Access: (0644/-rw-r--r--)  Uid: ( 1000/   error)   Gid: (  100/   users)
            Access: 2024-01-30 23:38:22.896900424 +0800
            Modify: 2024-01-30 23:38:22.589909282 +0800
            Change: 2024-01-30 23:38:22.893900510 +0800
             Birth: 2024-01-30 23:38:22.589909282 +0800
        ",
        );
        let mut input = owned_lines(
            "
              File: flake.nix
              Size: 1812      	Blocks: 8          IO Block: 4096   regular file
            Device: 0,68	Inode: 2522344     Links: 1
            Access: (0644/-rw-r--r--)  Uid: ( 1000/   error)   Gid: (  100/   users)
            Access: 2024-01-30 23:38:22.896900424 +0800
            Modify: 2024-01-30 23:38:22.589909282 +0800
            Change: 2024-01-30 23:38:22.893900510 +0800
             Birth: 2024-01-30 23:38:22.589909282 +0800
        ",
        );

        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        let regex = Regex::new(r"Size: (\d+).*IO Block: (\d+)").unwrap();
        let format = |size: u128| converter.format(size, 2);
        let output = replace(&mut input, &regex, &format, true);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }

    #[test]
    fn free() {
        let expected = owned_lines(
            "
                           total        used        free      shared     buffers       cache   available
            Mem:         16.7 GB      5.1 GB    823.5 MB      1.0 GB      1.3 MB     12.1 GB     11.6 GB
            Swap:        33.4 GB           1.0 B     33.3 GB
        ",
        );
        let mut input = owned_lines(
            "
                           total        used        free      shared     buffers       cache   available
            Mem:     16675958784  5125455872   823517184  1033273344     1310720 12103684096 11550502912
            Swap:    33351004160           1 33349431296
        ",
        );

        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        let regex = Regex::new(r"\d+").unwrap();
        let format = |size: u128| converter.format(size, 1);
        let output = replace(&mut input, &regex, &format, false);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }

    #[test]
    fn free_skip_short() {
        let expected = owned_lines(
            "
                           total        used        free      shared     buffers       cache   available
            Mem:         16.7 GB      5.1 GB    823.5 MB      1.0 GB      1.3 MB     12.1 GB     11.6 GB
            Swap:        33.4 GB           1     33.3 GB
        ",
        );
        let mut input = owned_lines(
            "
                           total        used        free      shared     buffers       cache   available
            Mem:     16675958784  5125455872   823517184  1033273344     1310720 12103684096 11550502912
            Swap:    33351004160           1 33349431296
        ",
        );

        let converter = Converter {
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        let regex = Regex::new(r"\d+").unwrap();
        let format = |size: u128| {
            converter.format_with_options(
                size,
                &Options {
                    precision: 1,
                    skip_short_numbers: true,
                    ..Options::default()
                },
            )
        };
        let output = replace(&mut input, &regex, &format, false);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }

    #[test]
    fn meminfo() {
        let expected = owned_lines(
            "
            MemTotal:       16.698 GB
            MemFree:          849.580 MB
            MemAvailable:   10.944 GB
            Buffers:              45.056 KB
            Cached:         10.882 GB
            SwapCached:         6.463 MB
            Active:          5.692 GB
            Inactive:        8.378 GB
            Active(anon):    3.860 GB
            Inactive(anon):   220.967 MB
            Active(file):    1.832 GB
            Inactive(file):  8.157 GB
            Unevictable:      302.432 MB
            Mlocked:             122.880 KB
            SwapTotal:      33.395 GB
            SwapFree:       32.269 GB
            NFS_Unstable:          0.000 B
            Bounce:                0.000 B
            WritebackTmp:          0.000 B
            CommitLimit:    41.744 GB
            Committed_AS:   11.782 GB
            VmallocTotal:   35.184 TB
            VmallocUsed:      284.680 MB
            VmallocChunk:          0.000 B
            Percpu:             6.849 MB
            HugePages_Rsvd:        0.000 B
            HugePages_Surp:        0.000 B
            Hugepagesize:       2.097 MB
            Hugetlb:               0.000 B
            DirectMap4k:     1.488 GB
            DirectMap2M:    14.533 GB
            DirectMap1G:     1.074 GB
        ",
        );
        let mut input = owned_lines(
            "
            MemTotal:       16306280
            MemFree:          829668
            MemAvailable:   10687284
            Buffers:              44
            Cached:         10627388
            SwapCached:         6312
            Active:          5558456
            Inactive:        8181964
            Active(anon):    3769684
            Inactive(anon):   215788
            Active(file):    1788772
            Inactive(file):  7966176
            Unevictable:      295344
            Mlocked:             120
            SwapTotal:      32612348
            SwapFree:       31513084
            NFS_Unstable:          0
            Bounce:                0
            WritebackTmp:          0
            CommitLimit:    40765488
            Committed_AS:   11505700
            VmallocTotal:   34359738367
            VmallocUsed:      278008
            VmallocChunk:          0
            Percpu:             6688
            HugePages_Rsvd:        0
            HugePages_Surp:        0
            Hugepagesize:       2048
            Hugetlb:               0
            DirectMap4k:     1453528
            DirectMap2M:    14192640
            DirectMap1G:     1048576
        ",
        );

        let converter = Converter {
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::K),
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        let regex = Regex::new(r"\d+$").unwrap();
        let format = |size: u128| converter.format(size, 3);
        let output = replace(&mut input, &regex, &format, true);

        assert_eq!(output.collect::<Vec<_>>(), expected.collect::<Vec<_>>());
    }
}
