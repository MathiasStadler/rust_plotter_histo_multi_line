// FROM HERE
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=dbadfff94a758f8aceaa75f884a3d9f1


// https://strawlab.org/strand-braid-api-docs/latest/chrono/struct.DateTime.html

use chrono::TimeZone;
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Utc, FixedOffset};

fn main() {
    let yyyy = 2024;
    let mm = 8;
    let dd = 16;
    let hour = 15;
    let min = 25;
    let time_zone = 7;

    let date = NaiveDate::from_ymd_opt(yyyy, mm, dd).unwrap();
    let time = NaiveTime::from_hms_opt(hour, min, 0).unwrap();
    let naive_datetime = NaiveDateTime::new(date, time);
    let my_tz = FixedOffset::east_opt(time_zone * 3600).unwrap();
    let local_datetime = naive_datetime.and_local_timezone(my_tz).unwrap();
    let utc_naive = local_datetime.naive_utc();
    let utc_datetime = Utc.from_local_datetime(&utc_naive);
    dbg!(naive_datetime);
    dbg!(local_datetime);
    dbg!(utc_naive);
    dbg!(utc_datetime);
}
