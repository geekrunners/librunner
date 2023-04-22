use chrono::Duration;
use geekrunners::Race;

fn main() {
    let duration = Duration::seconds(14400);

    let m_race: geekrunners::MetricRace = Race::new(42195, duration);

    println!("\nDistance: {}m, Duration: {:?}", m_race.distance, duration.num_seconds());
    println!("Pace (Km): {}:{}", m_race.average_pace().num_seconds() / 60, m_race.average_pace().num_seconds() % 60);
    println!("Laps: {:?}", m_race.laps().len());

    let i_race: geekrunners::ImperialRace = Race::new(46112, duration);

    println!("\nDistance: {}m, Duration: {:?}", i_race.distance, duration.num_seconds());
    println!("Pace (Mile): {}:{}", i_race.average_pace().num_seconds() / 60, i_race.average_pace().num_seconds() % 60);
    println!("Laps: {:?}", i_race.laps().len());
}