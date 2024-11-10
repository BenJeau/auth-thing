use std::time::{Duration, SystemTime};

pub trait SystemTimeExt {
    fn elapsed(&self) -> Duration;
}

impl SystemTimeExt for SystemTime {
    #[cfg_attr(test, mutants::skip)]
    fn elapsed(&self) -> Duration {
        self.duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards")
    }
}

pub fn get_number_of_steps(offset: u64, period: u64, time_fetcher: impl SystemTimeExt) -> u64 {
    let elapsed = time_fetcher.elapsed().as_secs();

    if elapsed < offset {
        return 0;
    }

    (elapsed - offset) / period
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockSystemTime(u64);

    impl SystemTimeExt for MockSystemTime {
        fn elapsed(&self) -> Duration {
            Duration::from_secs(self.0)
        }
    }

    #[test]
    fn test_given_offset_greater_than_current_time_then_number_of_steps_is_zero() {
        let offset = 1;
        let period = 30;
        let time_fetcher = MockSystemTime(0);

        let number_of_steps = get_number_of_steps(offset, period, time_fetcher);

        assert_eq!(number_of_steps, 0);
    }

    #[test]
    fn test_given_offset_equal_to_current_time_then_number_of_steps_is_zero() {
        let offset = 0;
        let period = 30;
        let time_fetcher = MockSystemTime(0);

        let number_of_steps = get_number_of_steps(offset, period, time_fetcher);

        assert_eq!(number_of_steps, 0);
    }

    #[test]
    fn test_given_time_elapsed_is_lower_than_offset_then_number_of_steps_is_zero() {
        let offset = 0;
        let period = 31;
        let time_fetcher = MockSystemTime(30);

        let number_of_steps = get_number_of_steps(offset, period, time_fetcher);

        assert_eq!(number_of_steps, 0);
    }

    #[test]
    fn test_given_time_elapsed_is_equal_to_offset_then_number_of_steps_is_one() {
        let offset = 0;
        let period = 30;
        let time_fetcher = MockSystemTime(30);

        let number_of_steps = get_number_of_steps(offset, period, time_fetcher);

        assert_eq!(number_of_steps, 1);
    }

    #[test]
    fn test_given_time_elapsed_is_multiple_periods_then_number_of_steps_is_correct() {
        let offset = 2;
        let period = 30;
        let time_fetcher = MockSystemTime(91);

        let number_of_steps = get_number_of_steps(offset, period, time_fetcher);

        assert_eq!(number_of_steps, 2);
    }
}
