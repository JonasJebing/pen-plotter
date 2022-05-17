# pen-plotter

An program for controling a simple pen plotter.
At the center the plotter has a stepper motor for rotation.
The distance of the pen from the center is determined by a second stepper motor
which drives a beam outwards or inwards.

## How to run pen-plotter

Use `cargo run -p pen-plotter -- -h` to print help information.
Use `cargo run -p pen-plotter -- --end 2,-10`
to have a basic idea what it would send to the stepper motors.

See https://rustup.rs/ for installing Rust and `cargo`.

## How the pen-plotter works

After parsing the command line arguments
pen-plotter spawns a thread for each motor.
The threads periodically calculate
on which step the motors should be to follow the path.
The interval for each thread is chosen
so that the motors could run at their maximum velocity.
If a motor needs to step forwards or backwards
pen-plotter sends that amount to the microcontroller.

The microcontroller immediately responds with the step number
that the motor will be on after stepping the given amount.
Then it controls the motor to perform the steps and waits for the next message.
The pen-plotter thread sleeps for the remaining interval
after receiving the current step number.
When the draw duration is elapsed, the threads stop.

The draw duration is calculated based on the path and the given velocity.
The path of the pen is calculated using linear interpolation
between the start and end coordinates.
Every interval the angle or extension of the beam
are calculated for the target point on the path.

## How pen-plotter communicates

1. Parsing the command line arguments using the `clap` library.
2. Sending a step delta to the motor microcontrollers
   as a signed 16 bit big endian integer.
3. Receiving the current step number from the motor microcontrollers
   as a signed 16 bit big endian integer.
