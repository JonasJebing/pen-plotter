use clap::Parser;
use eyre::{eyre, WrapErr};
use nalgebra::Point2;

const POINT_HELP: &str = "2D point in millimeter, for example 3,7 or 3.0,7.0";

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, help = POINT_HELP, parse(try_from_str = parse_point), default_value = "0,-10")]
    pub start: Point2<f64>,

    #[clap(short, long, help = POINT_HELP, parse(try_from_str = parse_point))]
    pub end: Point2<f64>,

    #[clap(long, help = "in mm/s", default_value = "1.2")]
    pub velocity: f64,
}

fn parse_point(s: &str) -> eyre::Result<Point2<f64>> {
    let (x, y) = s
        .split_once(',')
        .ok_or_else(|| eyre!("missing delimiter ',' in {s:?}"))?;
    Ok(Point2::new(
        x.parse()
            .wrap_err_with(|| format!("failed to parse x={x:?}"))?,
        y.parse()
            .wrap_err_with(|| format!("failed to parse y={y:?}"))?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_good_point() {
        let actual = parse_point("1,2").unwrap();
        let expected = Point2::new(1.0, 2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_decimal_point() {
        let actual = parse_point("4.2,9.5").unwrap();
        let expected = Point2::new(4.2, 9.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_bad_point() {
        let _ = parse_point("1, 2").unwrap_err();
    }
}
