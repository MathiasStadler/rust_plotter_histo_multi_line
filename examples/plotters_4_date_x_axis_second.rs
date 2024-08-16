// FROM HERE
// https://stackoverflow.com/questions/65975521/plotter-only-plot-the-time-not-the-date

use plotters::prelude::*;
// use chrono::{Utc, TimeZone};
// use chrono::offset;

use chrono::{Date, Duration, ParseError, NaiveTime};
use chrono::offset::{Utc,Local, TimeZone};
use chrono::Timelike;

// chrono::offset::Utc::now()

fn main() {
  let root_area = BitMapBackend::new("images/2.11.png", (600, 400))
    .into_drawing_area();
  root_area.fill(&WHITE).unwrap();

  // let start_date = Utc.ymd(2019, 10, 1);
  // let end_date = Utc.ymd(2019, 10, 18);
  //let start_date = Utc.ymd(2019, 10, 1);
  let start_date = Utc.with_ymd_and_hms(2019, 10, 8, 1, 0, 0).unwrap();
  // let end_date = Utc.ymd(2019, 10, 18);
  let end_date = Utc.with_ymd_and_hms(2019, 10, 18, 1, 0, 0).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("MSFT daily close price", ("sans-serif", 40))
        .build_cartesian_2d(start_date..end_date, 130.0..145.0)
        .unwrap();

    ctx.configure_mesh()
    .x_label_formatter(&|x| format!("{:02}:{:02}", x.hour(), x.minute()))
    .draw().unwrap();

    ctx.draw_series(
        LineSeries::new(
            (0..).zip(DATA.iter()).map(|(idx, price)| {
                let day = (idx / 5) * 7 + idx % 5 + 1;
                // let date = Utc.ymd(2019,10, day);
                let date = Utc.with_ymd_and_hms(2019, 10, day, 1, 0, 0).unwrap();
                (date, *price)
            }),
            &BLUE,
        )
    ).unwrap();

}
const DATA: [f64; 14] = [ 137.24, 136.37, 138.43, 137.41, 139.69, 140.41, 141.58, 139.55, 139.68, 139.10, 138.24, 135.67, 137.12, 138.12];