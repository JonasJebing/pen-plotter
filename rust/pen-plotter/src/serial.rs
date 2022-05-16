use crate::stepper::{CurrentStep, DeltaSteps};
use eyre::Context;
use serialport::SerialPort;
use std::{
    io::{Read, Write},
    time::Duration,
};

#[track_caller]
pub fn open(path: &str) -> eyre::Result<Box<dyn SerialPort>> {
    serialport::new(path, 9600)
        .timeout(Duration::from_secs(4))
        .open()
        .wrap_err_with(|| format!("failed to open serial port at {:?}", path))
}

pub fn write_steps<W: Write>(mut w: W, steps: DeltaSteps) -> eyre::Result<()> {
    w.write_all(&steps.0.to_be_bytes())
        .wrap_err_with(|| format!("failed to write {steps:?}"))
}

pub fn read_current_step<R: Read>(mut r: R) -> eyre::Result<CurrentStep> {
    let mut bytes = [0; 2];
    r.read_exact(&mut bytes)
        .wrap_err("failed to read current step")?;
    Ok(CurrentStep(i16::from_be_bytes(bytes)))
}
