use hsize::Converter;
use regex::Regex;
use std::io::{Error, Write};

/// # Errors
///
/// This function will return an error if it
/// fails to write to the destination buffer.
pub fn replace<T: Iterator<Item = String>>(
    input: T,
    output: &mut dyn Write,

    converter: &Converter,
    number_regex: &Regex,
) -> Result<(), Error> {
    for line in input {
        let mut new_line = line.clone();

        for number_capture in number_regex
            .captures_iter(&line)
            .map(|number_capture| {
                number_capture
                    .iter()
                    .collect::<Vec<_>>()
                    .iter()
                    .flat_map(std::borrow::ToOwned::to_owned)
                    .rev()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .iter()
            .rev()
        {
            for number_match in number_capture {
                if let Ok(number) = number_match.as_str().parse::<u128>() {
                    let converted_number = converter.format(number);
                    new_line.replace_range(number_match.range(), &converted_number);
                }
            }
        }

        writeln!(output, "{new_line}")?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::replace;
    use hsize::{Converter, Scale, Unit};
    use regex::Regex;

    #[test]
    fn single() {
        let expected = "19.028 PB".as_bytes().to_vec();

        let input = "19028310077231230";
        let mut output = Vec::new();

        let converter = Converter {
            precision: 3,
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        replace(
            input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &converter,
            &Regex::new(r"\d+").unwrap(),
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }

    #[test]
    fn multiple() {
        let expected = "12.06 KiB     1.00 B 1.00 B 1.00 B-------------1.00 MiB"
            .as_bytes()
            .to_vec();

        let input = "12345     1 1 1-------------1048576";
        let mut output = Vec::new();

        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: true,
                scale: None,
            },
        };
        replace(
            input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &converter,
            &Regex::new(r"\d+").unwrap(),
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }

    #[test]
    fn stat() {
        let expected = "
              File: flake.nix
              Size: 1.81 KB      	Blocks: 8          IO Block: 4.10 KB   regular file
            Device: 0,68	Inode: 2522344     Links: 1
            Access: (0644/-rw-r--r--)  Uid: ( 1000/   error)   Gid: (  100/   users)
            Access: 2024-01-30 23:38:22.896900424 +0800
            Modify: 2024-01-30 23:38:22.589909282 +0800
            Change: 2024-01-30 23:38:22.893900510 +0800
             Birth: 2024-01-30 23:38:22.589909282 +0800
        "
        .as_bytes()
        .to_vec();

        let input = "
              File: flake.nix
              Size: 1812      	Blocks: 8          IO Block: 4096   regular file
            Device: 0,68	Inode: 2522344     Links: 1
            Access: (0644/-rw-r--r--)  Uid: ( 1000/   error)   Gid: (  100/   users)
            Access: 2024-01-30 23:38:22.896900424 +0800
            Modify: 2024-01-30 23:38:22.589909282 +0800
            Change: 2024-01-30 23:38:22.893900510 +0800
             Birth: 2024-01-30 23:38:22.589909282 +0800
        ";
        let mut output = Vec::new();

        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                is_binary: false,
                scale: None,
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        replace(
            input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &converter,
            &Regex::new(r"Size: (\d+).*IO Block: (\d+)").unwrap(),
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }

    #[test]
    fn meminfo() {
        let expected = "
            MemTotal:       16.306 GB
            MemFree:          829.668 MB
            MemAvailable:   10.687 GB
            Buffers:              44.000 KB
            Cached:         10.627 GB
            SwapCached:         6.312 MB
            Active:          5.558 GB
            Inactive:        8.182 GB
            Active(anon):    3.770 GB
            Inactive(anon):   215.788 MB
            Active(file):    1.789 GB
            Inactive(file):  7.966 GB
            Unevictable:      295.344 MB
            Mlocked:             120.000 KB
            SwapTotal:      32.612 GB
            SwapFree:       31.513 GB
            NFS_Unstable:          0.000 KB
            Bounce:                0.000 KB
            WritebackTmp:          0.000 KB
            CommitLimit:    40.765 GB
            Committed_AS:   11.506 GB
            VmallocTotal:   34.360 TB
            VmallocUsed:      278.008 MB
            VmallocChunk:          0.000 KB
            Percpu:             6.688 MB
            HugePages_Rsvd:        0.000 KB
            HugePages_Surp:        0.000 KB
            Hugepagesize:       2.048 MB
            Hugetlb:               0.000 KB
            DirectMap4k:     1.454 GB
            DirectMap2M:    14.193 GB
            DirectMap1G:     1.049 GB
        "
        .as_bytes()
        .to_vec();

        let input = r"
            MemTotal:       16306280 kB
            MemFree:          829668 kB
            MemAvailable:   10687284 kB
            Buffers:              44 kB
            Cached:         10627388 kB
            SwapCached:         6312 kB
            Active:          5558456 kB
            Inactive:        8181964 kB
            Active(anon):    3769684 kB
            Inactive(anon):   215788 kB
            Active(file):    1788772 kB
            Inactive(file):  7966176 kB
            Unevictable:      295344 kB
            Mlocked:             120 kB
            SwapTotal:      32612348 kB
            SwapFree:       31513084 kB
            NFS_Unstable:          0 kB
            Bounce:                0 kB
            WritebackTmp:          0 kB
            CommitLimit:    40765488 kB
            Committed_AS:   11505700 kB
            VmallocTotal:   34359738367 kB
            VmallocUsed:      278008 kB
            VmallocChunk:          0 kB
            Percpu:             6688 kB
            HugePages_Rsvd:        0
            HugePages_Surp:        0
            Hugepagesize:       2048 kB
            Hugetlb:               0 kB
            DirectMap4k:     1453528 kB
            DirectMap2M:    14192640 kB
            DirectMap1G:     1048576 kB
        "
        .replace(" kB", "");
        let mut output = Vec::new();

        let converter = Converter {
            precision: 3,
            from_unit: Unit {
                is_binary: true,
                scale: Some(Scale::K),
            },
            to_unit: Unit {
                is_binary: false,
                scale: None,
            },
        };
        replace(
            input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &converter,
            &Regex::new(r"\d+$").unwrap(),
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }
}
