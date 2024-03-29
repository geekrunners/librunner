# librunner

Rust package to assist runners in planning workouts, completing races, and improving health.

## Usage

Let's go through these quick steps to get started with **librunner**:

1. visit https://rustup.rs and install **rustup**, an installer for the programming language Rust. Once installed, update and check the toolchain:

       $ rustup update
       $ rustc --version
       $ cargo --version

2. create your new running application:

       $ cargo new runningapp

3. a folder called `runningapp` is created. Go into it and run the project:

       $ cd runningapp
       $ cargo run

4. it prints "Hello World", meaning you have a working code to start from. Open the project in your favorite code editor and make two changes:

   4.1. add LibRunner to the project's dependencies:

       $ cargo add librunner

   It adds a new dependency to your `Cargo.toml` file:

      ```toml
      [dependencies]
      librunner = "0.7.0"
      ```

   4.2. replace the content of the file `src/main.rs` with the code below:

      ```rust
      use std::time::Duration;
      use librunner::running::Race;
      use librunner::running::Running;
      use librunner::running::MetricRace;
      use librunner::running::ImperialRace;
      use librunner::running::MetricRunning;
      use librunner::running::ImperialRunning;
      use librunner::distance;
      use librunner::duration;

      fn main() {
          let d = duration::to_duration(4, 0, 0); // 04:00:00
          let m_marathon: MetricRace = Race::new(42195);
          let m_running: MetricRunning = Running::new(d);

          println!("The pace to run {}km in {}h is approximately {}/km at {:.2}km/h", 
              distance::to_km(m_marathon.distance),
              duration::format_duration(m_running.duration()), 
              duration::format_duration(m_running.average_pace(&m_marathon)),
              distance::to_km_h(m_running.speed(&m_marathon)));

          let i_marathon: ImperialRace = Race::new(46112);
          let i_running: ImperialRunning = Running::new(d);

          println!("The pace to run {} miles in {}h is approximately {}/mile at {:.2}mph", 
              distance::to_mile(i_marathon.distance), 
              duration::format_duration(i_running.duration()),
              duration::format_duration(i_running.average_pace(&i_marathon)),
              distance::to_mph(i_running.speed(&i_marathon)));
      }
      ```
5. then run the project again:

       $ cargo run

   which generates the following output:

       The pace to run 42.195km in 04:00:00h is approximately 05:41/km at 10.55km/h
       The pace to run 26.2 miles in 04:00:00h is approximately 09:09/mile at 6.55mph

## License

**librunner** is used under the terms of the [Apache License version 2.0](https://github.com/geekrunners/librunner/blob/main/LICENSE).