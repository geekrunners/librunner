use chrono::Duration;

pub trait Race {
    const LAP_DISTANCE: i32;

    fn new(distance: i32, duration: Duration) -> Self;
    fn distance(&self) -> i32;
    fn duration(&self) -> Duration;
    
    fn average_pace(&self) -> Duration {
        return Duration::seconds((Self::LAP_DISTANCE as f32 * (self.duration().num_seconds() as f32 / self.distance() as f32)) as i64);
    }

    fn splits(&self) -> Vec<Duration> {
        let num_splits = self.distance() / Self::LAP_DISTANCE + if (self.distance() % Self::LAP_DISTANCE) > 0 { 1 } else { 0 };
        let mut splits = Vec::new();

        for _n in 0..num_splits {
            splits.push(self.average_pace());
        }

        return splits;
    }

    fn splits_with_elevation(&self) -> Vec<Duration> {
        self.splits()
    }
}

pub struct ImperialRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for ImperialRace {
    const LAP_DISTANCE: i32 = 1760;

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
    const LAP_DISTANCE: i32 = 1000;

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
    use chrono::Duration;

    use crate::Race;
    use crate::ImperialRace;
    use crate::MetricRace;

    #[test]
    fn test_metric_average_pace() {
        let duration = Duration::seconds(14400);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.average_pace().num_seconds(), 341);
        assert_eq!(m_race.average_pace().num_seconds() / 60, 5);
        assert_eq!(m_race.average_pace().num_seconds() % 60, 41);
    }

    #[test]
    fn test_metric_splits() {
        let duration = Duration::seconds(14400);
        let m_race: MetricRace = Race::new(42195, duration);
        assert_eq!(m_race.splits().len(), 43);
    }

    #[test]
    fn test_imperial_average_pace() {
        let duration = Duration::seconds(14400);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.average_pace().num_seconds(), 549);
        assert_eq!(i_race.average_pace().num_seconds() / 60, 9);
        assert_eq!(i_race.average_pace().num_seconds() % 60, 9);
    }

    #[test]
    fn test_imperial_splits() {
        let duration = Duration::seconds(14400);
        let i_race: ImperialRace = Race::new(46112, duration);
        assert_eq!(i_race.splits().len(), 27);
    }
}