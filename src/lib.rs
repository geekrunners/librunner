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
    /// use librunner::Race;
    /// use librunner::MetricRace;
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
    /// use librunner::Race;
    /// use librunner::MetricRace;
    /// 
    /// // Race measured in metric units
    /// let pace = Duration::new(341, 0); // seconds
    /// let m_race: MetricRace = Race::new_from_pace(42195, pace); // meters
    /// ```
    fn new_from_pace(distance: u64, pace: Duration) -> Self;

    /// Returns the distance of the race.
    fn distance(&self) -> u64;

    /// Returns the duration of the race.
    fn duration(&self) -> Duration;
    
    /// Calculates the average pace based on distance and duration.
    fn average_pace(&self) -> Duration {
        return Duration::new(
            (Self::SPLIT_DISTANCE as f32 * 
            (self.duration().as_secs() as f32 / self.distance() as f32)
        ) as u64, 0)
    }

    /// Calculates the speed of the runner to complete a distance within a duration.
    /// 
    /// Examples:
    /// 
    /// ```
    /// use std::time::Duration;
    /// use librunner::Race;
    /// use librunner::MetricRace;
    /// use librunner::ImperialRace;
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
    /// use librunner::Race;
    /// use librunner::MetricRace;
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

    fn new(distance: u64, duration: Duration) -> ImperialRace {
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
        let mut duration = ((i_race.distance() as f32 / Self::SPLIT_DISTANCE as f32) * pace.as_secs() as f32) as u64;
        duration += (((i_race.distance() as f32 % Self::SPLIT_DISTANCE as f32) * pace.as_secs() as f32) / Self::SPLIT_DISTANCE as f32) as u64;

        i_race.duration = Some(Duration::new(duration, 0));

        i_race
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
    /// use librunner::Race;
    /// use librunner::ImperialRace;
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

    fn new(distance: u64, duration: Duration) -> MetricRace {
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

        let mut duration = (m_race.distance() / Self::SPLIT_DISTANCE) * pace.as_secs();
        duration += ((m_race.distance() % Self::SPLIT_DISTANCE) * pace.as_secs()) / Self::SPLIT_DISTANCE;

        m_race.duration = Some(Duration::new(duration, 0));

        m_race
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Race;
    use crate::ImperialRace;
    use crate::MetricRace;

    #[test]
    fn test_new_metric_race() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.distance, 42195);
        assert_eq!(m_race.duration, Some(duration));

        let mp_race: MetricRace = Race::new_from_pace(42195, m_race.average_pace());
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

    #[test]
    fn test_new_imperial_race() {
        let duration = Duration::new(14400, 0);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.distance, 46112);
        assert_eq!(i_race.duration, Some(duration));

        let ip_race: ImperialRace = Race::new_from_pace(46112, i_race.average_pace());
        // The duration calculated from the pace correct, 
        // but there is a precision issue that needs to be addressed in the future.
        assert_eq!(ip_race.duration, Some(Duration::new(14492, 0)));
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
}