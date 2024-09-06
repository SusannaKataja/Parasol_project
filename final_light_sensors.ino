int sensorPin1 = A1;
int sensorPin2 = A2;
int sensorPin3 = A3;
int sensorPin4 = A4;
double lightData1 = 0;
double lightData2 = 0;
double lightData3 = 0;
double lightData4 = 0;

void setup() { // the setup function -- called once
  Serial.begin(9600);
}

void loop() {
  lightData1 = analogRead(sensorPin1);
  lightData2 = analogRead(sensorPin2);
  lightData3 = analogRead(sensorPin3);
  lightData4 = analogRead(sensorPin4);
  Serial.println(String(lightData1)+" "+String(lightData2)+" "+String(lightData3)+" "+String(lightData4));
  delay(1000);
}
