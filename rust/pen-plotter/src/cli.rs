use clap::Parser;
use eyre::{eyre, WrapErr};
use nalgebra::Point2;

const POINT_HELP: &str = "2D point in millimeter, for example (3,7) or (3.0,7.0)";

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(short, long, help = POINT_HELP, parse(try_from_str = parse_point), default_value = "(0,-10)")]
    pub start: Point2<f64>,

    #[clap(short, long, help = POINT_HELP, parse(try_from_str = parse_point))]
    pub end: Point2<f64>,

    #[clap(long, help = "in mm/s", default_value = "1.2")]
    pub velocity: f64,

    #[clap(long, help = "use this flag to perform I/O to the motors")]
    pub io: bool,

    #[clap(
        long,
        help = "device path for the central stepper motor",
        default_value = "/dev/ttyUSB0"
    )]
    pub central: String,

    #[clap(
        long,
        help = "device path for the beam stepper motor",
        default_value = "/dev/ttyUSB1"
    )]
    pub beam: String,
}

fn parse_point(s: &str) -> eyre::Result<Point2<f64>> {
    let (x, y) = s
        .strip_prefix('(')
        .ok_or_else(|| eyre!("missing prefix '(' in {s:?}"))?
        .strip_suffix(')')
        .ok_or_else(|| eyre!("missing suffix ')' in {s:?}"))?
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
        let actual = parse_point("(1,2)").unwrap();
        let expected = Point2::new(1.0, 2.0);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_decimal_point() {
        let actual = parse_point("(4.2,9.5)").unwrap();
        let expected = Point2::new(4.2, 9.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_bad_point() {
        let _ = parse_point("(1, 2)").unwrap_err();
    }

    #[test]
    fn parse_negative_x_coordinate() {
        Cli::try_parse_from(["bin-name", "--end", "(-1,0)"]).unwrap();
    }
}
