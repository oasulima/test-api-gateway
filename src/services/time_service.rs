use time::{Duration, OffsetDateTime, UtcOffset};

pub struct TimeService {
    day_data_cleanup_time_utc: Duration,
}

impl TimeService {
    pub fn new(day_data_cleanup_time_utc: Duration) -> Self {
        Self {
            day_data_cleanup_time_utc,
        }
    }

    pub fn get_previous_cleanup_time_in_utc(&self, now: OffsetDateTime) -> OffsetDateTime {
        let mut previous_cleanup =
            now.date().midnight().assume_offset(UtcOffset::UTC) + self.day_data_cleanup_time_utc;
        if previous_cleanup > now {
            previous_cleanup -= Duration::DAY;
        }

        previous_cleanup
    }
}
