// FROM HERE
// https://stackoverflow.com/questions/65937447/how-to-plot-a-series-with-a-date-on-the-x-axis-and-time-on-the-y-axis

// #![feature(allocator_api)]

use chrono::{Date, Duration, ParseError, NaiveTime};
use chrono::offset::{Local, TimeZone};
use plotters::prelude::*;

fn parse_datetime(t: &str) -> Date<Local> {
    Local
        .datetime_from_str(&format!("{} 0:0", t), "%Y-%m-%d %H:%M")
        .unwrap()
        .date()
}

fn parse_time(t: &str) -> Result<Duration, ParseError> {
    return match Local.datetime_from_str(&format!("2020-01-01 0:{}", t), "%Y-%m-%d %H:%M:%S%.f") {
        Ok(date) => Ok(date.time().signed_duration_since(NaiveTime::from_hms(0, 0, 0))),
        Err(e) => { println!("{}", e); Err(e) },
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = get_data();
    let root = BitMapBackend::new("images/stock-example.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let (from_date, to_date) = (
        parse_datetime(&data[0].0) - Duration::days(1),
        parse_datetime(&data[data.len() - 1].0) + Duration::days(1),
    );

    let y_min = parse_time("9:30.0").unwrap();
    let y_max = parse_time("13:00.0").unwrap();

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption("Time", ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_min..y_max)?;

    chart.configure_mesh()
        .light_line_style(&WHITE)
        .y_label_formatter(&|y| format!("{:02}:{:02}", y.num_minutes(), y.num_seconds() % 60))
        .x_label_formatter(&|x| x.naive_local().to_string())
        .draw()?;

    chart.draw_series(
        data.iter()
            .map(|x| Circle::new((parse_datetime(x.0), parse_time(x.1).unwrap()), 5, BLUE.filled())),
    )?;

    Ok(())
}

fn get_data() -> Vec<(&'static str, &'static str, f32, f32, f32)> {
    return vec![
        ("2019-04-18", "10:11.5", 16.0, 121.3018, 123.3700),
        ("2019-04-22", "10:52.2", 15.0, 122.5700, 123.7600),
        ("2019-04-23", "12:23.5", 14.0, 123.8300, 125.4400),
        ("2019-04-24", "10:15.0", 13.0, 124.5200, 125.0100),
        ("2019-04-25", "10:43.9", 12.0, 128.8300, 129.1500),
    ];
}

// cargo run --example