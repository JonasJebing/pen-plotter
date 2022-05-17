#include <Stepper.h>

const int stepsPerRevolution = 400;

Stepper stepper(stepsPerRevolution, 8, 9, 10, 11);

short stepCount = 0;

void setup() {
  Serial.begin(9600);
}

void loop() {
  if (Serial.available() > 0) {
    byte steps0 = Serial.read();
    byte steps1 = Serial.read();
    short steps = beToShort(steps0, steps1);
    stepCount += steps;
    stepCount = stepCount % stepsPerRevolution;
    byte stepCount0 = shortToBe0(stepCount);
    byte stepCount1 = shortToBe1(stepCount);
    Serial.write(stepCount0);
    Serial.write(stepCount1);
    stepper.step(steps);
    // read stepCount from motor if available
  }
}

/// convert big endian bytes to short and vice versa
/// TODO: not sure if arduino uses little endian and if implementation is correct
short beToShort(byte a, byte b) {
  return ((unsigned short) (a << 8)) | b;
}
byte shortToBe0(short x) {
  return (byte) (x >> 8);
}
byte shortToBe1(short x) {
  return (byte) x;
}
