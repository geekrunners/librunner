# Lib Runner

Rust package to assist runners in planning workouts, completing races, and improving health.

## Usage

Let's go through these quick steps to get started with LibRunner:

1. visit https://rustup.rs and install rustup, an installer for the programming language Rust. Once installed, update and check the toolchain:

       $ rustup update
       $ rustc --version
       $ cargo --version

2. create your new running application:

       $ cargo new runningapp

3. a folder called `runningapp` is created. Go into it and run the project:

       $ cd runningapp
       $ cargo run

4. it prints "Hello World", meaning you have a working code to start from. Open the project in your favourite code editor and make two changes:

   4.1. add LibRunner to the project's dependencies:

       $ cargo add librunner

   It adds a new dependency to your `Cargo.toml` file:

      ```toml
      [dependencies]
      librunner = "0.6.0"
      ```

   4.2. replace the content of the file `src/main.rs` with the code below:

      ```rust
      use std::time::Duration;
      use librunner::running::{Race, MetricRace, ImperialRace};
      use librunner::utils::converter;
      use librunner::utils::formatter;

      fn main() {
          let duration = converter::to_duration(4, 0, 0); // 04:00:00
          let m_race: MetricRace = Race::new(42195, duration);

          println!("The pace to run {}km in {}h is approximately {}/km at {:.2}km/h", 
                   converter::to_km(m_race.distance),
                   formatter::format_duration(m_race.duration()), 
                   formatter::format_duration(m_race.average_pace()),
                   converter::to_km_h(m_race.speed()));

          let i_race: ImperialRace = Race::new(46112, duration);

          println!("The pace to run {} miles in {}h is approximately {}/mile at {:.2}mph", 
                   converter::to_mile(i_race.distance), 
                   formatter::format_duration(i_race.duration()),
                   formatter::format_duration(i_race.average_pace()),
                   converter::to_mph(i_race.speed()));
      }
      ```
5. then run the project again:

       $ cargo run

   which generates the following output:

       The pace to run 42.195km in 04:00:00h is approximately 05.41/km at 10.55km/h
       The pace to run 26.2 miles in 04:00:00h is approximately 09.09/mile at 11.53mph

## License

LibRunner is used under the terms of the [Apache License version 2.0](https://github.com/geekrunners/librunner/blob/main/LICENSE).