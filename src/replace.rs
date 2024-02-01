use regex::Regex;
use std::io::{Error, Write};

/// # Errors
///
/// This function will return an error if it
/// fails to write to the destination buffer.
pub fn replace(
    input: &mut (impl Iterator<Item = String> + ?Sized),
    output: &mut dyn Write,

    format_fn: &dyn Fn(u128) -> String,
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
                    new_line.replace_range(number_match.range(), &format_fn(number));
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
            &mut input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &|size: u128| converter.format(size, 3),
            &Regex::new(r"\d+").unwrap(),
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap().trim(),
            String::from_utf8(expected).unwrap().trim()
        );
    }

    #[test]
    fn multiple() {
        let expected = "12.06 KiB     1.00 B 1.00 B 1.00 B-------------1.00 MiB"
            .as_bytes()
            .to_vec();

        let input = "12345     1 1 1-------------1048576";
        let mut output = Vec::new();

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
        replace(
            &mut input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &|size: u128| converter.format(size, 2),
            &Regex::new(r"\d+").unwrap(),
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap().trim(),
            String::from_utf8(expected).unwrap().trim()
        );
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
            &mut input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &|size: u128| converter.format(size, 2),
            &Regex::new(r"Size: (\d+).*IO Block: (\d+)").unwrap(),
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap().trim(),
            String::from_utf8(expected).unwrap().trim()
        );
    }

    #[test]
    fn meminfo() {
        let expected = "
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
            &mut input.lines().map(std::borrow::ToOwned::to_owned),
            &mut output,
            &|size: u128| converter.format(size, 3),
            &Regex::new(r"\d+$").unwrap(),
        )
        .unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap().trim(),
            String::from_utf8(expected).unwrap().trim()
        );
    }
}
