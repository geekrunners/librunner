/// Utility functions to convert, format and do other things with values.
pub mod utils {
    /// Converts values between units and types.
    pub mod converter {
        use std::time::Duration;

        /// Creates a Duration based on the arguments hours, minutes, and seconds.
        ///
        /// Example:
        ///
        /// ```
        /// use librunner::utils::converter;
        ///
        /// let duration = converter::to_duration(4, 5, 19); // 04:05:19
        /// assert_eq!(duration.as_secs(), 14719);
        /// ```
        pub fn to_duration(hours: u64, minutes: u64, seconds: u64) -> Duration {
            let mins = if hours > 0 { hours * 60 } else { 0 } + minutes;
            let secs = if mins > 0 { mins * 60 } else { 0 } + seconds;
            Duration::new(secs, 0)
        }

        #[cfg(test)]
        mod tests {
            use crate::utils::converter;

            #[test]
            fn test_to_duration() {
                let duration = converter::to_duration(4, 5, 19);
                assert_eq!(duration.as_secs(), 14719);
            }
        }
    }

    /// Formats values to make them human-readable.
    pub mod formatter {
        use std::time::Duration;

        /// Formats a duration to a human readable text.
        ///
        /// Example:
        ///
        /// ```
        /// use librunner::utils::formatter;
        /// use librunner::utils::converter;
        ///
        /// let duration = converter::to_duration(4, 5, 19);
        /// println!("Duration: {}", formatter::format_duration(duration));
        /// ```
        ///
        /// It prints "Duration: 04:05:19".
        pub fn format_duration(duration: Duration) -> String {
            let mut secs = duration.as_secs();
            let mut mins = 0;
            let mut hors = 0;

            if secs >= 60 {
                mins = secs / 60;
                secs = secs % 60;
                hors = mins / 60;
                mins = mins % 60;
            }
            format!("{:02}:{:02}:{:02}", hors, mins, secs)
        }

        #[cfg(test)]
        mod tests {
            use crate::utils::converter;
            use crate::utils::formatter;

            #[test]
            fn test_format_duration() {
                assert_eq!(formatter::format_duration(converter::to_duration(0, 0, 0)), "00:00:00");
                assert_eq!(formatter::format_duration(converter::to_duration(0, 0, 9)), "00:00:09");
                assert_eq!(formatter::format_duration(converter::to_duration(0, 5, 9)), "00:05:09");
                assert_eq!(formatter::format_duration(converter::to_duration(4, 5, 19)), "04:05:19");
                assert_eq!(formatter::format_duration(converter::to_duration(135, 59, 1)), "135:59:01");
            }
        }
    }
}

/// API to make running calculations.
pub mod running {
    use std::time::Duration;

    /// A running race, already with common calculations that work with multiple scales.
    pub trait Race {
        /// The distance of one split in an implemented scale.
        const SPLIT_DISTANCE: u64;

        /// Creates a new instance of the race with the supported attributes.
        /// 
        /// Example:
        /// 
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::Race;
        /// use librunner::running::MetricRace;
        /// 
        /// // Race measured in metric units
        /// let duration = Duration::new(14400, 0); // seconds
        /// let m_race: MetricRace = Race::new(42195, duration); // meters
        /// ```
        fn new(distance: u64, duration: Duration) -> Self;

        /// Creates a new instance of the race using the desired pace to calculate the duration.
        /// 
        /// Example:
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::Race;
        /// use librunner::running::MetricRace;
        /// 
        /// // Race measured in metric units
        /// let pace = Duration::new(341, 0); // seconds
        /// let m_race: MetricRace = Race::new_from_pace(42195, pace); // meters
        /// ```
        fn new_from_pace(distance: u64, pace: Duration) -> Self;

        fn new_from_splits(splits: Vec<Duration>) -> Self;

        /// Returns the distance of the race.
        fn distance(&self) -> u64;

        /// Returns the duration of the race.
        fn duration(&self) -> Duration;
        
        /// Calculates the average pace based on distance and duration.
        /// 
        /// Examples:
        /// 
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::{Race, ImperialRace, MetricRace};
        /// 
        /// let duration = Duration::new(14400, 0);
        /// 
        /// // Imperial marathon race. Average pace: 9:09/mile
        /// let i_race: ImperialRace = Race::new(46112, duration);
        /// assert_eq!(i_race.average_pace().as_secs(), 549);
        /// assert_eq!(i_race.average_pace().as_secs() / 60, 9);
        /// assert_eq!(i_race.average_pace().as_secs() % 60, 9);
        /// 
        /// // Metric marathon race. Average pace: 5:41/km
        /// let m_race: MetricRace = Race::new(42195, duration);
        /// assert_eq!(m_race.average_pace().as_secs(), 341);
        /// assert_eq!(m_race.average_pace().as_secs() / 60, 5);
        /// assert_eq!(m_race.average_pace().as_secs() % 60, 41);
        /// ```
        fn average_pace(&self) -> Duration {
            return Duration::new(
                (Self::SPLIT_DISTANCE as f32 * (self.duration().as_secs() as f32 / self.distance() as f32)
            ) as u64, 0)
        }

        /// Calculates the speed of the runner to complete a distance within a duration.
        /// 
        /// Examples:
        /// 
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::Race;
        /// use librunner::running::MetricRace;
        /// use librunner::running::ImperialRace;
        /// 
        /// // Race measured in metric units
        /// let duration = Duration::new(14400, 0); // seconds
        /// let m_race: MetricRace = Race::new(42195, duration); // meters
        /// assert_eq!(m_race.speed(), 2.9302084); // m/s
        /// 
        /// // Race measured in imperial units
        /// let i_race: ImperialRace = Race::new(46112, duration); // yards
        /// assert_eq!(i_race.speed(), 3.202222); // yd/s
        /// ```
        fn speed(&self) -> f32 {
            self.distance() as f32 / self.duration().as_secs() as f32
        }

        /// Calculates the number of splits based on the race distance and the split distance.
        /// The split distance is defined in each Race implementation. 1 km is a tipical example of split.
        /// 
        /// Example:
        /// 
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::Race;
        /// use librunner::running::MetricRace;
        /// 
        /// let duration = Duration::new(14400, 0);
        /// let m_race: MetricRace = Race::new(42195, duration);
        /// assert_eq!(m_race.num_splits(), 43);
        /// ```
        fn num_splits(&self) -> u64 {
            self.distance() / Self::SPLIT_DISTANCE + if (self.distance() % Self::SPLIT_DISTANCE) > 0 { 1 } else { 0 }
        }

        /// Returns the splits of the race, with the average pace in each split.
        fn splits(&self) -> Vec<Duration> {
            let average_pace = self.average_pace();
            self.splits_with_pace(average_pace)
        }

        /// Returns the splits of the race with a custom pace.
        fn splits_with_pace(&self, pace: Duration) -> Vec<Duration> {
            let mut splits = Vec::new();
            
            for _n in 0..self.num_splits() {
                splits.push(pace);
            }

            splits
        }

        /// Returns the splits of the race from a higher to a lower pace, according to the degree of variation.
        /// 
        /// # Arguments
        /// 
        /// * `degree` - the degree of variation from the average pace in seconds.
        fn negative_splits(&self, degree: Duration) -> Vec<Duration> {
            // minutes between minimal and maximum pace
            let variation = (2 * degree.as_secs()) + 1;
            let num_splits = self.num_splits();
            // size of the block of splits with the same pace
            let block = num_splits / variation;
            let average_pace = self.average_pace();

            let mut negative_splits = Vec::new();
            // the pace starts high and decrements at every splits block
            let mut pace = Duration::new(average_pace.as_secs() + degree.as_secs(), 0);
            let mut block_count = 0;
            
            for _n in 0..num_splits as usize {
                if block == block_count {
                    // decrements the pace at every new block.
                    let secs = pace.as_secs() - 1u64;
                    pace = Duration::new(secs, 0);

                    block_count = 0;
                }
                negative_splits.push(pace);
                block_count += 1;
            }

            negative_splits
        }

        /// Returns the splits of the race from a lower to a higher pace, according to the degree of variation.
        /// 
        /// # Arguments
        /// 
        /// * `degree` - the degree of variation from the average pace in seconds.
        fn positive_splits(&self, degree: Duration) -> Vec<Duration> {
            let variation = (2 * degree.as_secs()) + 1;
            let num_splits = self.num_splits();
            // size of the block of splits with the same pace
            let block = num_splits / variation;
            let average_pace = self.average_pace();

            let mut positive_splits = Vec::new();
            // the pace starts high and decrements at every splits block
            let mut pace = Duration::new(average_pace.as_secs() - degree.as_secs(), 0);
            let mut block_count = 0;
            
            for _n in 0..num_splits as usize {
                if block == block_count {
                    // decrements the pace at every new block.
                    let secs = pace.as_secs() + 1u64;
                    pace = Duration::new(secs, 0);

                    block_count = 0;
                }
                positive_splits.push(pace);
                block_count += 1;
            }

            positive_splits
        }
    }

    /// A running race using the imperial scale, such as miles and yards.
    pub struct ImperialRace {
        pub distance: u64,
        pub duration: Option<Duration>
    }

    impl Race for ImperialRace {
        const SPLIT_DISTANCE: u64 = 1760; // yards

        fn new(distance: u64, duration: Duration) -> Self {
            ImperialRace {
                distance,
                duration: Some(duration)
            }
        }

        fn new_from_pace(distance: u64, pace: Duration) -> Self {
            // Creates an imperial race without duration
            let mut i_race = ImperialRace {
                distance,
                duration: None
            };
            let duration = (i_race.distance() as f32 / Self::SPLIT_DISTANCE as f32) * pace.as_secs() as f32;
            
            i_race.duration = Some(Duration::new(duration as u64, 0));

            i_race
        }

        fn new_from_splits(splits: Vec<Duration>) -> Self {
            let distance = splits.len() as u64 * Self::SPLIT_DISTANCE;
            
            let mut duration = 0;
            for split in splits {
                duration += split.as_secs();
            }

            ImperialRace {
                distance,
                duration: Some(Duration::new(duration, 0))
            }
        }

        fn distance(&self) -> u64 {
            self.distance
        }

        fn duration(&self) -> Duration {
            match self.duration {
                Some(p) => p,
                None => Duration::new(0, 0)
            }
        }
    }

    impl ImperialRace {
        /// Calculates the speed of the runner in miles per hour (mph).
        /// ```
        /// use std::time::Duration;
        /// use librunner::running::Race;
        /// use librunner::running::ImperialRace;
        /// 
        /// // Race measured in imperial units
        /// let duration = Duration::new(14400, 0); // seconds
        /// let i_race: ImperialRace = Race::new(46112, duration); // yards
        /// assert_eq!(i_race.speed_miles_hour(), 6.55); // mph
        /// ```
        pub fn speed_miles_hour(&self) -> f32 {
            let miles = self.distance() as f32 / 1760.0;
            miles / (self.duration().as_secs() as f32 / 60.0 / 60.0)
        }
    }

    /// A running race using the metric scale, such as kilometers and metters.
    pub struct MetricRace {
        pub distance: u64,
        pub duration: Option<Duration>
    }

    impl Race for MetricRace {
        const SPLIT_DISTANCE: u64 = 1000; // meters

        fn new(distance: u64, duration: Duration) -> Self {
            MetricRace {
                distance,
                duration: Some(duration)
            }
        }

        fn new_from_pace(distance: u64, pace: Duration) -> Self {
            let mut m_race = MetricRace {
                distance,
                duration: None
            };

            let duration = (m_race.distance() as f32 / Self::SPLIT_DISTANCE as f32) * pace.as_secs() as f32;
            
            m_race.duration = Some(Duration::new(duration as u64, 0));

            m_race
        }

        fn new_from_splits(splits: Vec<Duration>) -> Self {
            let distance = splits.len() as u64 * Self::SPLIT_DISTANCE;
            
            let mut duration = 0;
            for split in splits {
                duration += split.as_secs();
            }

            MetricRace {
                distance,
                duration: Some(Duration::new(duration, 0))
            }
        }

        fn distance(&self) -> u64 {
            self.distance
        }

        fn duration(&self) -> Duration {
            match self.duration {
                Some(p) => p,
                None => Duration::new(0, 0)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::running::Race;
    use crate::running::ImperialRace;
    use crate::running::MetricRace;

    #[test]
    fn test_new_imperial_race() {
        let duration = Duration::new(14400, 0);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.distance, 46112);
        assert_eq!(i_race.duration, Some(duration));
    }

    #[test]
    fn test_new_imperial_from_pace() {
        let ip_race: ImperialRace = Race::new_from_pace(46112, Duration::new(549, 0));
        // The duration calculated from the pace correct, 
        // but there is a precision issue that needs to be addressed in the future.
        assert_eq!(ip_race.duration, Some(Duration::new(14383, 0)));
    }

    #[test]
    fn test_imperial_average_pace() {
        let duration = Duration::new(14400, 0);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.average_pace().as_secs(), 549);
        assert_eq!(i_race.average_pace().as_secs() / 60, 9);
        assert_eq!(i_race.average_pace().as_secs() % 60, 9);
    }

    #[test]
    fn test_imperial_num_splits() {
        let duration = Duration::new(14400, 0);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.num_splits(), 27);
    }

    #[test]
    fn test_imperial_splits_duration() {
        let duration = Duration::new(14400, 0);
        let i_race: ImperialRace = Race::new(46112, duration);
        let splits = i_race.splits();
        let average_pace = i_race.average_pace();

        for split in splits {
            assert_eq!(split, average_pace);
        }
    }

    #[test]
    fn test_new_metric_race() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.distance, 42195);
        assert_eq!(m_race.duration, Some(duration));
    }

    #[test]
    fn test_new_metric_from_pace() {
        let mp_race: MetricRace = Race::new_from_pace(42195, Duration::new(341, 0));
        // The duration calculated from the pace correct, 
        // but there is a precision issue that needs to be addressed in the future.
        assert_eq!(mp_race.duration, Some(Duration::new(14388, 0)));
    }

    #[test]
    fn test_metric_average_pace() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.average_pace().as_secs(), 341);
        assert_eq!(m_race.average_pace().as_secs() / 60, 5);
        assert_eq!(m_race.average_pace().as_secs() % 60, 41);
    }

    #[test]
    fn test_metric_num_splits() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.num_splits(), 43);
    }

    #[test]
    fn test_metric_splits_duration() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        let splits = m_race.splits();
        let average_pace = m_race.average_pace();

        for split in splits {
            assert_eq!(split, average_pace);
        }
    }

    #[test]
    fn test_metric_negative_splits() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        
        let degree = Duration::new(5, 0);
        let variation = (2 * degree.as_secs()) + 1;
        let block = m_race.num_splits() / variation;
        let negative_splits = m_race.negative_splits(degree);

        assert_eq!(negative_splits[0].as_secs(), 346);
        assert_eq!(negative_splits[block as usize].as_secs(), 346 - 1);
        assert_eq!(negative_splits[block as usize * 2].as_secs(), 346 - 2);
        assert_eq!(negative_splits[block as usize * variation as usize].as_secs(), 346 - variation as u64);
        assert_eq!(negative_splits[block as usize * degree.as_secs() as usize].as_secs(), m_race.average_pace().as_secs());
    }

    #[test]
    fn test_metric_positive_splits() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        
        let degree = Duration::new(5, 0);
        let variation = (2 * degree.as_secs()) + 1;
        let block = m_race.num_splits() / variation;
        let positive_splits = m_race.positive_splits(degree);

        assert_eq!(positive_splits[0].as_secs(), 346 - (degree.as_secs() * 2) as u64);
        assert_eq!(positive_splits[block as usize].as_secs(), 346 - (degree.as_secs() * 2) as u64 + 1);
        assert_eq!(positive_splits[block as usize * 2].as_secs(), 346 - (degree.as_secs() * 2) as u64 + 2);
        assert_eq!(positive_splits[block as usize * variation as usize].as_secs(), 346 + 1);
        assert_eq!(positive_splits[block as usize * degree.as_secs() as usize].as_secs(), m_race.average_pace().as_secs());
    }
}