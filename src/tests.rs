pub mod bench_from_ascii {
    use test::Bencher;

    use convert::FromAscii;
    //the benches use .cloned(), since iterating over an array of &str's gives &&str's.

    const STRING_U16: [&str; 25] = ["9874", "2983", "18", "8734", "94", "83", "9873", "64999", "83", "19", "945", "3278",
                                    "9845", "387", "8390", "92", "9", "1", "2", "3", "45", "842", "987", "9874", "83"];

    const STRING_U32: [&str; 25] = ["987245987", "876249", "1786235", "987234", "8723095", "786349276", "8763240", "83638730",
                                    "730372", "5628493", "6712398", "987234", "28764", "7", "2", "98724", "9", "10", "123", "83",
                                    "287", "178", "372", "876", "7019"];

    #[bench]
    fn bench_u16_from_ascii(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(STRING_U16.iter().cloned().map(u16::atoi).all(|n| n.is_ok()), true);
        })
    }

    #[bench]
    fn bench_u16_std_parse(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(STRING_U16.iter().cloned().map(|s| s.parse::<u16>()).all(|n| n.is_ok()), true)
        })
    }

    #[bench]
    fn bench_u32_from_ascii(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(STRING_U32.iter().cloned().map(u32::atoi).all(|n| n.is_ok()), true);
        })
    }

    #[bench]
    fn bench_u32_std_parse(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(STRING_U32.iter().cloned().map(|s| s.parse::<u32>()).all(|n| n.is_ok()), true)
        })
    }
}

pub mod bench_into_ascii {
    use test::Bencher;

    use convert::IntoAscii;

    const INT_U16: [u16; 25] = [234, 4356, 234, 356, 567, 345, 2345, 456, 5467, 234, 234, 5436, 567, 345, 456, 5467, 234, 234, 456, 234, 23, 45, 456, 34, 45];
    const INT_U64: [u64; 25] = [982374, 987234, 98456, 136, 2354, 3, 8743, 9645, 2173, 0986, 237493984, 79613, 34, 98274, 965, 2, 5, 2, 8, 98274, 987768234, 9875, 987, 9878376, 1678];

    #[bench]
    fn bench_u16_into_ascii(b: &mut Bencher) {
        b.iter(|| {
            for item in INT_U16.iter() {
                item.itoa();
            }
        })
    }

    #[bench]
    fn bench_u16_vec_sd(b: &mut Bencher) {
        b.iter(|| {
            for item in INT_U16.iter() {
                format!("{}", item).into_bytes();
            }
        })
    }

    #[bench]
    fn bench_u64_into_ascii(b: &mut Bencher) {
        b.iter(|| {
            for item in INT_U64.iter() {
                item.itoa();
            }
        })
    }

    #[bench]
    fn bench_u64_vec_sd(b: &mut Bencher) {
        b.iter(|| {
            for item in INT_U64.iter() {
                format!("{}", item).into_bytes();
            }
        })
    }
}