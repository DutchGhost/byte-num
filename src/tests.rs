#[cfg(feature = "nightly")]
pub mod bench_from_ascii {
    use test;
    use test::Bencher;

    use convert::FromAscii;
    //the benches use .cloned(), since iterating over an array of &str's gives &&str's.

    const STRING_U16: [&str; 25] = [
        "9874", "2983", "18", "8734", "94", "83", "9873", "64999", "83", "19", "945", "3278",
        "9845", "387", "8390", "92", "9", "1", "2", "3", "45", "842", "987", "9874", "83",
    ];

    const STRING_U32: [&str; 25] = [
        "987245987",
        "876249",
        "1786235",
        "987234",
        "8723095",
        "786349276",
        "8763240",
        "83638730",
        "730372",
        "5628493",
        "6712398",
        "987234",
        "28764",
        "7",
        "2",
        "98724",
        "9",
        "10",
        "123",
        "83",
        "287",
        "178",
        "372",
        "876",
        "7019",
    ];

    const STRING_U64: [&str; 25] = [
        "987245987",
        "876249",
        "1786235",
        "987234",
        "87230945",
        "786349276",
        "8763240",
        "83638730",
        "730372",
        "5628493",
        "6712398",
        "987234",
        "28764",
        "7982375",
        "2982874",
        "98724",
        "9928374",
        "198740",
        "128833",
        "88373",
        "2898747",
        "17837368",
        "37235362",
        "872346",
        "7019",
    ];

    #[bench]
    fn bench_u64_from_ascii(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U64
                    .iter()
                    .cloned()
                    .cycle()
                    .take(10_000)
                    .map(u64::atoi)
                    .all(|n| n.is_ok()),
                true
            );
        })
    }

    #[bench]
    fn bench_u64_from_ascii_std(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U64
                    .iter()
                    .cloned()
                    .cycle()
                    .take(10_000)
                    .map(|n| n.parse::<u64>())
                    .all(|n| n.is_ok()),
                true
            );
        })
    }

    #[cfg(feature = "with_exact")]
    #[bench]
    fn bench_u64_from_ascii_exact(b: &mut Bencher) {
        use from_ascii as EXACT;
        b.iter(|| {
            assert_eq!(
                STRING_U64
                    .iter()
                    .cloned()
                    .cycle()
                    .take(10_000)
                    .map(<u64 as EXACT::FromAscii>::atoi)
                    .all(|n| n.is_ok()),
                true
            );
        })
    }

    #[bench]
    fn bench_u64_from_ascii_unchecked(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                test::black_box(unsafe {
                    STRING_U64
                        .iter()
                        .cloned()
                        .map(|b| u64::atoi_unchecked(b))
                        .for_each(|_| {})
                }),
                ()
            );
        })
    }

    #[bench]
    fn bench_u16_from_ascii(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U16.iter().cloned().map(u16::atoi).all(|n| n.is_ok()),
                true
            );
        })
    }

    #[bench]
    fn bench_u16_std_parse(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U16
                    .iter()
                    .cloned()
                    .map(|s| s.parse::<u16>())
                    .all(|n| n.is_ok(),),
                true
            )
        })
    }

    #[bench]
    fn bench_u32_from_ascii(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U32.iter().cloned().map(u32::atoi).all(|n| n.is_ok()),
                true
            );
        })
    }

    #[bench]
    fn bench_u32_std_parse(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(
                STRING_U32
                    .iter()
                    .cloned()
                    .map(|s| s.parse::<u32>())
                    .all(|n| n.is_ok(),),
                true
            )
        })
    }
}

pub mod bench_into_ascii {

    use convert::IntoAscii;

    const INT_U16: [u16; 25] = [
        234, 4356, 234, 356, 567, 345, 2345, 456, 5467, 234, 234, 5436, 567, 345, 456, 5467, 234,
        234, 456, 234, 23, 45, 456, 34, 45,
    ];

    #[cfg(feature = "nightly")]
    const INT_U64: [u64; 25] = [
        982374, 987234, 98456, 136, 2354, 3, 8743, 9645, 2173, 0986, 237493984, 79613, 34, 98274,
        965, 2, 5, 2, 8, 98274, 987768234, 9875, 987, 9878376, 1678,
    ];

    #[cfg(feature = "nightly")]
    const INT_U32: [u32; 100] = [
        1128170912, 1426521144, 2584211806, 3033901428, 3679896292, 30393242, 2194237445,
        3496945018, 1518198753, 2998518165, 3483791564, 285949496, 652585618, 378575128,
        3146207847, 3326002999, 4149225605, 995360151, 3389156678, 850927266, 1351915052,
        2773769492, 1834900655, 2274011986, 1122109790, 692166123, 2785145886, 776503336,
        170169736, 854130356, 3573581178, 581363774, 4080770407, 2279135774, 5221036, 3239775968,
        1443397413, 4234196902, 3271180626, 925222097, 207687252, 3489281117, 2312922534,
        2888420501, 1682599680, 1222101966, 3691329295, 1164669231, 2420262085, 3629222729,
        3511006590, 2455351840, 3507437695, 3456802434, 2982495865, 1945777526, 4069596950,
        3787366909, 443961781, 4019820499, 3718378946, 4012713142, 3781160855, 9423066, 3867352068,
        1621092314, 1758348053, 2468556791, 3738894631, 1875094074, 809544647, 606751253,
        4174356743, 3867608610, 1195337723, 4162238628, 1955064931, 3345458282, 1855081754,
        843953635, 3090571767, 654172748, 2521149887, 2218701234, 2635228471, 1739056009,
        764134121, 1564358706, 664762184, 2855483101, 794706100, 2273887742, 3674615768, 783111593,
        1736522292, 508412023, 142384014, 2519138932, 618595379, 441221996,
    ];

    #[test]
    fn negative_itoa() {
        assert_eq!({ -10 }.itoa(), [b'-', b'1', b'0']);
        assert_eq!(0i32.itoa(), [b'0']);
    }

    #[test]
    fn test_for_equality() {
        let itoa_vec = INT_U16.iter().map(|n| n.itoa()).collect::<Vec<_>>();

        let format_vec = INT_U16
            .iter()
            .map(|n| format!("{}", n).into_bytes())
            .collect::<Vec<_>>();

        assert_eq!(itoa_vec, format_vec);
    }

    #[cfg(feature = "nightly")]
    pub mod benches {
        use convert::IntoAscii;

        use test::Bencher;
        use tests::bench_into_ascii::{INT_U16, INT_U32, INT_U64};

        #[bench]
        fn bench_u16_into_ascii(b: &mut Bencher) {
            b.iter(|| {
                for item in INT_U16.iter() {
                    item.itoa();
                }
            })
        }

        #[bench]
        fn bench_u16_vec_std(b: &mut Bencher) {
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
        fn bench_u64_vec_std(b: &mut Bencher) {
            b.iter(|| {
                for item in INT_U64.iter() {
                    format!("{}", item).into_bytes();
                }
            })
        }

        #[bench]
        fn bench_100_u32_into_ascii(b: &mut Bencher) {
            b.iter(|| {
                for item in INT_U32.iter() {
                    item.itoa();
                }
            })
        }

        #[bench]
        fn bench_100_u32_vec_std(b: &mut Bencher) {
            b.iter(|| {
                for item in INT_U32.iter() {
                    format!("{}", item).into_bytes();
                }
            })
        }

        #[cfg(feature = "with_exact")]
        #[bench]
        fn bench_100_u32_into_ascii_with_exact(b: &mut Bencher) {
            use into_ascii as ITOA;

            b.iter(|| {
                for item in INT_U32.iter() {
                    <u32 as ITOA::IntoAscii>::itoa(item);
                }
            })
        }
    }
}
