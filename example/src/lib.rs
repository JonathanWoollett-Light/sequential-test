#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use sequential_test::{parallel, sequential};
    use std::{thread, time::Duration};
    // use serial_test::{serial,parallel};

    const MAX_DURATION: Duration = Duration::from_millis(10);
    const FAILURE_RATE: u8 = 0; // 0..100
                                // Every test sleep for 0..MAX_DURATION then has a FAILURE_RATE % change to fail.
    fn test() {
        let mut rng = thread_rng();
        let duration = rng.gen_range(Duration::ZERO..MAX_DURATION);
        thread::sleep(duration);
        assert!(rng.gen_range(0..100) >= FAILURE_RATE);
    }
    #[test]
    #[sequential]
    fn test1s() {
        test();
    }
    #[test]
    #[sequential]
    fn test2s() {
        test();
    }
    #[test]
    #[sequential]
    fn test3s() {
        test();
    }
    #[test]
    #[sequential]
    fn test4s() {
        test();
    }
    #[test]
    #[sequential]
    fn test5s() {
        test();
    }
    #[test]
    #[sequential]
    fn test6s() {
        test();
    }
    #[test]
    #[sequential]
    fn test7s() {
        test();
    }
    #[test]
    #[sequential]
    fn test8s() {
        test();
    }
    #[test]
    #[sequential]
    fn test9s() {
        test();
    }
    #[test]
    #[sequential]
    fn test10s() {
        test();
    }
    #[test]
    #[sequential]
    fn test11s() {
        test();
    }
    #[test]
    #[sequential]
    fn test12s() {
        test();
    }
    #[test]
    #[sequential]
    fn test13s() {
        test();
    }
    #[test]
    #[sequential]
    fn test14s() {
        test();
    }
    #[test]
    #[sequential]
    fn test15s() {
        test();
    }
    #[test]
    #[parallel]
    fn test1p() {
        test();
    }
    #[test]
    #[parallel]
    fn test2p() {
        test();
    }
    #[test]
    #[parallel]
    fn test3p() {
        test();
    }
    #[test]
    #[parallel]
    fn test4p() {
        test();
    }
    #[test]
    #[parallel]
    fn test5p() {
        test();
    }
    #[test]
    #[parallel]
    fn test6p() {
        test();
    }
    #[test]
    #[parallel]
    fn test7p() {
        test();
    }
    #[test]
    #[parallel]
    fn test8p() {
        test();
    }
    #[test]
    #[parallel]
    fn test9p() {
        test();
    }
    #[test]
    #[parallel]
    fn test10p() {
        test();
    }
    #[test]
    #[parallel]
    fn test11p() {
        test();
    }
    #[test]
    #[parallel]
    fn test12p() {
        test();
    }
    #[test]
    #[parallel]
    fn test13p() {
        test();
    }
    #[test]
    #[parallel]
    fn test14p() {
        test();
    }
    #[test]
    #[parallel]
    fn test15p() {
        test();
    }
    #[test]
    #[parallel]
    fn test16p() {
        test();
    }
    #[test]
    #[parallel]
    fn test17p() {
        test();
    }
    #[test]
    #[parallel]
    fn test18p() {
        test();
    }
    #[test]
    #[parallel]
    fn test19p() {
        test();
    }
    #[test]
    #[parallel]
    fn test20p() {
        test();
    }
    #[test]
    #[parallel]
    fn test21p() {
        test();
    }
    #[test]
    #[parallel]
    fn test22p() {
        test();
    }
    #[test]
    #[parallel]
    fn test23p() {
        test();
    }
    #[test]
    #[parallel]
    fn test24p() {
        test();
    }
    #[test]
    #[parallel]
    fn test25p() {
        test();
    }
    #[test]
    #[parallel]
    fn test26p() {
        test();
    }
    #[test]
    #[parallel]
    fn test27p() {
        test();
    }
    #[test]
    #[parallel]
    fn test28p() {
        test();
    }
    #[test]
    #[parallel]
    fn test29p() {
        test();
    }
    #[test]
    #[parallel]
    fn test30p() {
        test();
    }
}
