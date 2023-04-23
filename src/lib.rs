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