# Lib Runner

Rust library to assist runners on planning their workouts, races, and improve their health.

## Usage

Run the following Cargo command in your project folder:

    $ cargo add librunner

Call it in the source code:

```rust
use std::time::Duration;
use librunner::running::{Race, MetricRace, ImperialRace};
use librunner::utils::convert;

fn main() {
    let duration = convert::to_duration(4, 0, 0); // 04:00:00
    let m_race: MetricRace = Race::new(42195, duration);
    let m_average_pace = m_race.average_pace();

    println!("The pace to run {}km in {}h is approximately {}.{}/km at {:.2}km/h", 
             (m_race.distance as f32 / 1000.0), 
             (duration.as_secs() / 60 / 60), 
             (m_average_pace.as_secs() / 60),
             (m_average_pace.as_secs() % 60),
             (m_race.speed() * 3.6));

    let i_race: ImperialRace = Race::new(46112, duration);
    let i_average_pace = i_race.average_pace();

    println!("The pace to run {} miles in {}h is approximately {}.{}/mile at {:.2}mph", 
             (i_race.distance as f32 / 1760.0), 
             (duration.as_secs() / 60 / 60),
             (i_average_pace.as_secs() / 60),
             (i_average_pace.as_secs() % 60),
             (i_race.speed() * 3.6));
}
```
