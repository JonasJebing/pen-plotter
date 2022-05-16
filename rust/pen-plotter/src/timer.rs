use std::{
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IntervalTimer {
    pub start: Instant,
    pub duration: Duration,
    pub interval: Duration,
}

impl IntervalTimer {
    pub fn for_each_fraction<F>(self, mut f: F) -> eyre::Result<()>
    where
        F: FnMut(f64) -> eyre::Result<()>,
    {
        let mut step_start = self.start;
        loop {
            let elapsed = step_start.duration_since(self.start);
            let fraction = fraction(elapsed, self.duration);
            f(fraction)?;
            if elapsed > self.duration {
                return Ok(());
            }
            thread::sleep(self.interval.saturating_sub(step_start.elapsed()));
            step_start = Instant::now();
        }
    }
}

fn fraction(left: Duration, right: Duration) -> f64 {
    left.as_secs_f64() / right.as_secs_f64()
}
