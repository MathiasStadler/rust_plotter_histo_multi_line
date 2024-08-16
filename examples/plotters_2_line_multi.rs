// FROM HERE
// https://plotters-rs.github.io/book/basic/basic_data_plotting.html

use plotters::prelude::*;

fn main() {
    let root_area = BitMapBackend::new("images/2.10_multi_line.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Histo + Line", ("sans-serif", 40))
        .build_cartesian_2d(0..10, 0..80)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let data = [25, 37, 15, 32, 45, 33, 32, 10, 0, 21];

    let data_two = [27, 31, 17, 37, 47, 37, 37, 18, 8, 27];

    // Draw the histogram
    // ctx.draw_series((0..).zip(data.iter()).map(|(x, y)| {
    //     let mut bar = Rectangle::new([(x, 0), (x + 1, *y)], GREEN.filled());
    //     bar.set_margin(0, 0, 5, 5);
    //     bar
    // }))
    // .unwrap();

    // Draw the line series
    ctx.draw_series(LineSeries::new(
        (0..).zip(data.iter()).map(|(x, y)| (x, *y + 30)),
        &BLUE,
    ))
    .unwrap();

    // Draw the line series
    ctx.draw_series(LineSeries::new(
        (0..).zip(data_two.iter()).map(|(x, y)| (x, *y + 30)),
        &BLUE,
    ))
    .unwrap();

}
