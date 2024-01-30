use hsize::Converter;
use regex::RegexBuilder;
use std::io::Write;

#[derive(Debug)]
pub enum Error {
    Regex(String),
    Write(std::io::Error),
}

/// # Errors
///
/// This function will return an error if the
/// regex is invalid or if a write error occurs.
pub fn replace<T: Iterator<Item = String>>(
    input: T,
    output: &mut dyn Write,

    converter: &Converter,
    number_regex_string: &str,
    multiline: bool,
) -> Result<(), Error> {
    let number_regex = match RegexBuilder::new(number_regex_string)
        .multi_line(multiline)
        .build()
    {
        Ok(number_regex) => number_regex,
        Err(error) => return Err(Error::Regex(error.to_string())),
    };

    for line in input {
        let mut new_line = line.clone();
        for number_match in number_regex
            .find_iter(&line)
            .collect::<Vec<_>>()
            .iter()
            .rev()
        {
            if let Ok(number) = number_match.as_str().parse::<u128>() {
                let converted_number = converter.convert(number);
                new_line.replace_range(number_match.range(), &converted_number);
            }
        }

        new_line.push('\n');
        if let Err(error) = output.write(new_line.as_bytes()) {
            return Err(Error::Write(error));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::replace;
    use hsize::{Converter, Scale, Unit};

    #[test]
    fn replace_single() {
        let expected = "19.02831 PB".as_bytes().to_vec();

        let input = "19028310077231230";
        let mut output = Vec::new();

        let converter = Converter {
            precision: 5,
            from_unit: Unit {
                binary: false,
                scale: None,
            },
            to_unit: Unit {
                binary: false,
                scale: None,
            },
        };
        replace(
            input.lines().map(|line| line.to_owned()),
            &mut output,
            &converter,
            r"\d+",
            false,
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }

    #[test]
    fn replace_multiple() {
        let expected = "12.06 KiB     1.00 B 1.00 B 1.00 B-------------1.00 MiB"
            .as_bytes()
            .to_vec();

        let input = "12345     1 1 1-------------1048576";
        let mut output = Vec::new();

        let converter = Converter {
            precision: 2,
            from_unit: Unit {
                binary: false,
                scale: None,
            },
            to_unit: Unit {
                binary: true,
                scale: None,
            },
        };
        replace(
            input.lines().map(|line| line.to_owned()),
            &mut output,
            &converter,
            r"\d+",
            false,
        )
        .unwrap();
        output.pop();

        println!("{}", std::str::from_utf8(&output).unwrap());

        assert_eq!(output, expected);
    }

    #[test]
    fn replace_meminfo() {
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
            Zswap:                 0.000 KB
            Zswapped:              0.000 KB
            Dirty:               660.000 KB
            Writeback:             0.000 KB
            AnonPages:       3.404 GB
            Mapped:           667.800 MB
            Shmem:            872.484 MB
            KReclaimable:     468.088 MB
            Slab:             800.908 MB
            SReclaimable:     468.088 MB
            SUnreclaim:       332.820 MB
            KernelStack:       21.216 MB
            PageTables:        49.988 MB
            SecPageTables:         0.000 KB
            NFS_Unstable:          0.000 KB
            Bounce:                0.000 KB
            WritebackTmp:          0.000 KB
            CommitLimit:    40.765 GB
            Committed_AS:   11.506 GB
            VmallocTotal:   34.360 TB
            VmallocUsed:      278.008 MB
            VmallocChunk:          0.000 KB
            Percpu:             6.688 MB
            AnonHugePages:         0.000 KB
            ShmemHugePages:        0.000 KB
            ShmemPmdMapped:        0.000 KB
            FileHugePages:         0.000 KB
            FilePmdMapped:         0.000 KB
            CmaTotal:              0.000 KB
            CmaFree:               0.000 KB
            Unaccepted:            0.000 KB
            HugePages_Total:       0.000 KB
            HugePages_Free:        0.000 KB
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
            Zswap:                 0 kB
            Zswapped:              0 kB
            Dirty:               660 kB
            Writeback:             0 kB
            AnonPages:       3403652 kB
            Mapped:           667800 kB
            Shmem:            872484 kB
            KReclaimable:     468088 kB
            Slab:             800908 kB
            SReclaimable:     468088 kB
            SUnreclaim:       332820 kB
            KernelStack:       21216 kB
            PageTables:        49988 kB
            SecPageTables:         0 kB
            NFS_Unstable:          0 kB
            Bounce:                0 kB
            WritebackTmp:          0 kB
            CommitLimit:    40765488 kB
            Committed_AS:   11505700 kB
            VmallocTotal:   34359738367 kB
            VmallocUsed:      278008 kB
            VmallocChunk:          0 kB
            Percpu:             6688 kB
            AnonHugePages:         0 kB
            ShmemHugePages:        0 kB
            ShmemPmdMapped:        0 kB
            FileHugePages:         0 kB
            FilePmdMapped:         0 kB
            CmaTotal:              0 kB
            CmaFree:               0 kB
            Unaccepted:            0 kB
            HugePages_Total:       0
            HugePages_Free:        0
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
                binary: true,
                scale: Some(Scale::K),
            },
            to_unit: Unit {
                binary: false,
                scale: None,
            },
        };
        replace(
            input.lines().map(|line| line.to_owned()),
            &mut output,
            &converter,
            r"\d+$",
            false,
        )
        .unwrap();
        output.pop();

        assert_eq!(output, expected);
    }
}
