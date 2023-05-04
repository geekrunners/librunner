use std::time::Duration;

pub trait Race {
    const SPLIT_DISTANCE: i32;

    fn new(distance: i32, duration: Duration) -> Self;
    fn distance(&self) -> i32;
    fn duration(&self) -> Duration;
    
    fn average_pace(&self) -> Duration {
        return Duration::new((Self::SPLIT_DISTANCE as f32 * (self.duration().as_secs() as f32 / self.distance() as f32)) as u64, 0);
    }

    fn speed(&self) -> f32 {
        self.distance() as f32 / self.duration().as_secs() as f32
    }

    fn num_splits(&self) -> i32 {
        self.distance() / Self::SPLIT_DISTANCE + if (self.distance() % Self::SPLIT_DISTANCE) > 0 { 1 } else { 0 }
    }

    fn splits(&self) -> Vec<Duration> {
        let mut splits = Vec::new();
        let average_pace = self.average_pace();

        for _n in 0..self.num_splits() {
            splits.push(average_pace);
        }

        return splits;
    }

    fn negative_splits(&self, degree: usize) -> Vec<Duration> {
        // minutes between minimal and maximum pace
        let variation = (2 * degree) + 1;
        let num_splits = self.num_splits();
        // size of the block of splits with the same pace
        let block = num_splits as usize / variation;
        let average_pace = self.average_pace();

        let mut negative_splits = Vec::new();
        // the pace starts high and decrements at every splits block
        let mut pace = Duration::new(average_pace.as_secs() + degree as u64, 0);
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

        return negative_splits;
    }

    fn positive_splits(&self, degree: usize) -> Vec<Duration> {
        let variation = (2 * degree) + 1;
        let num_splits = self.num_splits();
        // size of the block of splits with the same pace
        let block = num_splits as usize / variation;
        let average_pace = self.average_pace();

        let mut positive_splits = Vec::new();
        // the pace starts high and decrements at every splits block
        let mut pace = Duration::new(average_pace.as_secs() - degree as u64, 0);
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

        return positive_splits;
    }
}

pub struct ImperialRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for ImperialRace {
    const SPLIT_DISTANCE: i32 = 1760; // yards

    fn new(distance: i32, duration: Duration) -> ImperialRace {
        ImperialRace {
            distance,
            duration
        }
    }

    fn distance(&self) -> i32 {
        self.distance
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

pub struct MetricRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for MetricRace {
    const SPLIT_DISTANCE: i32 = 1000; // meters

    fn new(distance: i32, duration: Duration) -> MetricRace {
        MetricRace {
            distance,
            duration
        }
    }

    fn distance(&self) -> i32 {
        self.distance
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::Race;
    use crate::ImperialRace;
    use crate::MetricRace;

    #[test]
    fn test_metric_average_pace() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.average_pace().as_secs(), 341);
        assert_eq!(m_race.average_pace().as_secs() / 60, 5);
        assert_eq!(m_race.average_pace().as_secs() % 60, 41);
    }

    #[test]
    fn test_metric_speed() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.speed(), 2.9302084);
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
        
        let degree = 5;
        let variation = (2 * degree) + 1;
        let block = m_race.num_splits() as usize / variation;
        let negative_splits = m_race.negative_splits(degree);

        assert_eq!(negative_splits[0].as_secs(), 346);
        assert_eq!(negative_splits[block].as_secs(), 346 - 1);
        assert_eq!(negative_splits[block * 2].as_secs(), 346 - 2);
        assert_eq!(negative_splits[block * variation].as_secs(), 346 - variation as u64);
        assert_eq!(negative_splits[block * degree].as_secs(), m_race.average_pace().as_secs());
    }

    #[test]
    fn test_metric_positive_splits() {
        let duration = Duration::new(14400, 0);
        let m_race: MetricRace = Race::new(42195, duration);
        
        let degree = 5;
        let variation = (2 * degree) + 1;
        let block = m_race.num_splits() as usize / variation;
        let positive_splits = m_race.positive_splits(degree);

        assert_eq!(positive_splits[0].as_secs(), 346 - (degree * 2) as u64);
        assert_eq!(positive_splits[block].as_secs(), 346 - (degree * 2) as u64 + 1);
        assert_eq!(positive_splits[block * 2].as_secs(), 346 - (degree * 2) as u64 + 2);
        assert_eq!(positive_splits[block * variation].as_secs(), 346 + 1);
        assert_eq!(positive_splits[block * degree].as_secs(), m_race.average_pace().as_secs());
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