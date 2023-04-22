use chrono::Duration;

pub trait Race {
    const LAP_DISTANCE: i32;

    fn new(distance: i32, duration: Duration) -> Self;
    fn distance(&self) -> i32;
    fn duration(&self) -> Duration;
    
    fn average_pace(&self) -> Duration {
        return Duration::seconds((Self::LAP_DISTANCE as f32 * (self.duration().num_seconds() as f32 / self.distance() as f32)) as i64);
    }

    fn laps(&self) -> Vec<Duration> {
        let num_laps = self.distance() / Self::LAP_DISTANCE + if (self.distance() % Self::LAP_DISTANCE) > 0 { 1 } else { 0 };
        let mut laps = Vec::new();

        for _n in 0..num_laps {
            laps.push(self.average_pace());
        }

        return laps;
    }
}

struct ImperialRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for ImperialRace {
    const LAP_DISTANCE: i32 = 1760;

    fn new(distance: i32, duration: Duration) -> ImperialRace {
        ImperialRace {
            distance: distance,
            duration: duration
        }
    }

    fn distance(&self) -> i32 {
        self.distance
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

struct MetricRace {
    pub distance: i32,
    pub duration: Duration
}

impl Race for MetricRace {
    const LAP_DISTANCE: i32 = 1000;

    fn new(distance: i32, duration: Duration) -> MetricRace {
        MetricRace {
            distance: distance,
            duration: duration
        }
    }

    fn distance(&self) -> i32 {
        self.distance
    }

    fn duration(&self) -> Duration {
        self.duration
    }
}

fn main() {
    let duration = Duration::seconds(14400);

    let m_race: MetricRace = Race::new(42195, duration);

    println!("\nDistance: {}m, Duration: {:?}", m_race.distance, duration.num_seconds());
    println!("Pace (Km): {}:{}", m_race.average_pace().num_seconds() / 60, m_race.average_pace().num_seconds() % 60);
    println!("Laps: {:?}", m_race.laps().len());

    let i_race: ImperialRace = Race::new(46112, duration);

    println!("\nDistance: {}m, Duration: {:?}", i_race.distance, duration.num_seconds());
    println!("Pace (Mile): {}:{}", i_race.average_pace().num_seconds() / 60, i_race.average_pace().num_seconds() % 60);
    println!("Laps: {:?}", i_race.laps().len());
}