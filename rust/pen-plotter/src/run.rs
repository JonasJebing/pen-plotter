use crate::{
    cli::Cli,
    motion::Motion,
    named::Named,
    path::{Path, Point},
    stepper::{
        beam::BeamStepper, central::CentralStepper, CurrentStep, DeltaSteps, Stepper, TargetStep,
    },
    timer::IntervalTimer,
};
use clap::Parser;
use log::LevelFilter;
use std::{
    fmt::{self, Debug},
    panic::Location,
    process,
    thread::{self, JoinHandle},
    time::Instant,
};

pub fn run() {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .parse_default_env()
        .init();
    if let Err(e) = try_run() {
        let report: eyre::Report = e;
        log::error!("{report:?}")
    }
}

fn try_run() -> eyre::Result<()> {
    let cli = Cli::parse();
    let motion = Motion {
        path: path_from_cli(&cli),
        start: Instant::now(),
    };
    let join_handles = [
        spawn_step_through_motion(motion, CentralStepper::default(), log_interval),
        spawn_step_through_motion(motion, BeamStepper::default(), log_interval),
    ];
    for h in join_handles {
        exit_on_error(h.join());
    }
    Ok(())
}

fn path_from_cli(cli: &Cli) -> Path {
    Path {
        start: cli.start,
        end: cli.end,
        velocity: cli.velocity,
    }
}

fn spawn_step_through_motion<S, F>(motion: Motion, stepper: S, f: F) -> JoinHandle<()>
where
    S: Stepper + Named + Send + 'static,
    F: FnMut(IntervalContext) -> eyre::Result<CurrentStep> + Send + 'static,
{
    thread::spawn(move || exit_on_error(step_through_motion(motion, stepper, f)))
}

fn step_through_motion<S, F>(motion: Motion, stepper: S, mut f: F) -> eyre::Result<()>
where
    S: Stepper + Named,
    F: FnMut(IntervalContext) -> eyre::Result<CurrentStep>,
{
    let Motion { path, start } = motion;
    let interval_timer = IntervalTimer {
        start,
        duration: path.duration()?,
        interval: stepper.min_step_interval(),
    };
    let mut current_step = CurrentStep(0);
    interval_timer.for_each_fraction(|fraction| {
        let target = motion.path.interpolate(fraction);
        let target_step = stepper.target_step(target)?;
        let delta_steps = stepper.delta_steps(current_step, target_step)?;
        current_step = f(IntervalContext {
            stepper_name: stepper.name(),
            point: target,
            target_step,
            delta_steps,
        })?;
        Ok(())
    })
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct IntervalContext {
    pub stepper_name: &'static str,
    pub point: Point,
    pub target_step: TargetStep,
    pub delta_steps: DeltaSteps,
}

impl fmt::Display for IntervalContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: delta = {}, target = {}, location = {}",
            self.stepper_name, self.delta_steps.0, self.target_step.0, self.point.location
        )
    }
}

fn log_interval(interval: IntervalContext) -> eyre::Result<CurrentStep> {
    if interval.delta_steps.0 != 0 {
        log::info!("{interval}");
    }
    Ok(CurrentStep(interval.target_step.0))
}

#[track_caller]
fn exit_on_error<T, E>(result: Result<T, E>)
where
    E: Debug,
{
    if let Err(e) = result {
        let location = Location::caller();
        log::error!("exit at {location} caused by error {e:?}");
        process::exit(-1)
    }
}
