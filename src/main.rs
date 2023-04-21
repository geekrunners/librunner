use chrono::Duration;

pub trait Race {
    fn new(distance: i32, duration: Duration) -> Self;
    fn pace(&self) -> Duration;
    fn laps(&self) -> Vec<Duration>;
}

struct ImperialRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for ImperialRace {
    fn new(distance: i32, duration: Duration) -> ImperialRace {
        ImperialRace {
            distance: distance,
            duration: duration
        }
    }

    fn pace(&self) -> Duration {
        return Duration::seconds((1760.0 * (self.duration.num_seconds() as f32 / self.distance as f32)) as i64);
    }

    fn laps(&self) -> Vec<Duration> {
        let num_laps = self.distance / 1760 + if (self.distance % 1760) > 0 { 1 } else { 0 };
        let mut laps = Vec::new();

        for _n in 0..num_laps {
            laps.push(self.pace());
        }

        return laps;
    }
}

struct MetricRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for MetricRace {
    fn new(distance: i32, duration: Duration) -> MetricRace {
        MetricRace {
            distance: distance,
            duration: duration
        }
    }

    fn pace(&self) -> Duration {
        return Duration::seconds((1000.0 * (self.duration.num_seconds() as f32 / self.distance as f32)) as i64);
    }

    fn laps(&self) -> Vec<Duration> {
        let num_laps = self.distance / 1000 + if (self.distance % 1000) > 0 { 1 } else { 0 };
        let mut laps = Vec::new();

        for _n in 0..num_laps {
            laps.push(self.pace());
        }

        return laps;
    }
}

fn main() {
    let duration = Duration::seconds(14400);

    let m_race: MetricRace = Race::new(42195, duration);

    println!("\nDistance: {}m, Duration: {:?}, Pace: {:?}", m_race.distance, duration.num_seconds(), m_race.pace().num_seconds());
    println!("Pace (Km): {}:{}", m_race.pace().num_seconds() / 60, m_race.pace().num_seconds() % 60);
    println!("Laps: {:?}", m_race.laps().len());

    let i_race: ImperialRace = Race::new(46112, duration);

    println!("\nDistance: {}m, Duration: {:?}, Pace: {:?}", i_race.distance, duration.num_seconds(), i_race.pace().num_seconds());
    println!("Pace (Mile): {}:{}", i_race.pace().num_seconds() / 60, i_race.pace().num_seconds() % 60);
    println!("Laps: {:?}", i_race.laps().len());
}