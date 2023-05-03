# Lib Runner

Rust library to assist runners on planning their workouts, races, and improve their health.

## Usage

Add Lib Runner as a dependency to your `cargo.toml`:

```toml
[dependencies]
librunner = "0.1.0"
```

Call it from the source code:

```rust
use std::time::Duration;
use librunner::{Race, MetricRace};

fn main() {
    // 14400 is the number of seconds within 4 hours
    let duration = Duration::new(14400, 0);

    // 42195 is the distance in metters of a marathon
    let m_race: MetricRace = Race::new(42195, duration);

    let average_pace = m_race.average_pace();

    println!("The pace to run {}km in {}h is {}.{}/km", 
             (m_race.distance / 1000), 
             (duration.as_secs() / 60 / 60), 
             (average_pace.as_secs() / 60),
             (average_pace.as_secs() % 60));
}
```

## Contribute

### Build

    $ cargo build

### Test

    $ cargo test