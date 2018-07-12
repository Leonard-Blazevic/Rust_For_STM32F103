#include <ESP8266WiFi.h>
#include <BlynkSimpleEsp8266.h>
#include <IRremoteESP8266.h>
#include <IRsend.h>

#define IR_LED 2  // ESP8266 GPIO pin to use. Recommended: 4 (D2).

IRsend irsend(IR_LED);  // Set the GPIO to be used to sending the message.

uint16_t rawDataOn[23] = {1250, 400, 1300, 400, 450, 1200, 1300, 400, 1300, 400, 400, 1250, 450, 1250, 450, 1200, 450, 1250, 450, 1200, 1300, 400, 450};
uint16_t rawDataOff[23] = {1250, 400, 1300, 400, 400, 1300, 1250, 400, 1250, 400, 450, 1250, 400, 1250, 450, 1250, 400, 1250, 450, 1250, 400, 1300, 1250};

uint16_t turnOn[67] = {9050, 4450, 550, 550, 600, 500, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 600, 500, 600, 1700, 550, 1650, 550, 1650, 600, 1700, 500, 550, 600, 1650, 550, 1650, 600, 1650, 600, 1650, 550, 1700, 500, 600, 550, 550, 600, 500, 600, 550, 550, 550, 600, 500, 600, 550, 550, 550, 600, 1650, 550, 1650, 550, 1700, 550, 1700, 550, 1650, 550, 1650, 600};
uint16_t turnOff[67] = {9050, 4400, 600, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 1650, 550, 1650, 600, 1600, 650, 1600, 600, 550, 550, 1650, 600, 1650, 550, 1650, 600, 550, 550, 1650, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 1650, 550, 500, 650, 1650, 550, 1650, 550, 1650, 600, 1650, 600, 1600, 600, 1650, 600};
uint16_t turnRed[67] = {9050, 4400, 550, 550, 600, 550, 550, 550, 600, 550, 550, 550, 600, 500, 550, 600, 550, 500, 650, 1650, 550, 1600, 600, 1650, 600, 1650, 550, 550, 600, 1650, 550, 1650, 600, 1650, 600, 500, 600, 550, 550, 1650, 550, 550, 600, 500, 600, 550, 550, 600, 550, 550, 550, 1650, 600, 1600, 600, 550, 600, 1650, 550, 1650, 600, 1600, 600, 1650, 550, 1650, 600};
uint16_t turnGreen[67] = {9000, 4400, 600, 550, 600, 500, 600, 500, 600, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 1650, 550, 1650, 600, 1650, 600, 1600, 600, 500, 600, 1650, 600, 1600, 600, 1650, 600, 1600, 600, 550, 600, 1650, 550, 500, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 600, 1650, 550, 550, 550, 1650, 600, 1650, 550, 1700, 550, 1650, 550, 1650, 600};
uint16_t turnBlue[67] = {9000, 4450, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 550, 1700, 550, 1700, 500, 1700, 550, 1700, 500, 550, 600, 1650, 550, 1700, 500, 1700, 550, 550, 550, 1700, 550, 1700, 500, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 1650, 550, 550, 550, 600, 550, 1700, 500, 1700, 550, 1700, 500, 1700, 550, 1650, 550};
uint16_t turnOrange[67] = {9000, 4400, 600, 550, 550, 550, 600, 500, 600, 550, 550, 550, 600, 500, 600, 550, 550, 550, 550, 1700, 550, 1700, 500, 1650, 600, 1650, 550, 550, 600, 1650, 550, 1650, 550, 1700, 550, 550, 550, 600, 550, 550, 550, 550, 550, 1650, 600, 550, 550, 550, 550, 550, 600, 1650, 550, 1700, 550, 1650, 550, 1700, 550, 550, 550, 1700, 550, 1650, 550, 1700, 550};
uint16_t flash[67] = {9000, 4400, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 1700, 550, 1650, 550, 1700, 550, 1700, 500, 550, 600, 1650, 550, 1650, 550, 1700, 550, 1700, 550, 1650, 550, 550, 550, 1700, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 1650, 600, 500, 550, 1700, 550, 1700, 550, 1650, 550, 1700, 550};
uint16_t fade[67] = {9000, 4400, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 550, 550, 600, 550, 550, 550, 1700, 550, 1650, 550, 1700, 550, 1600, 600, 550, 600, 1650, 550, 1650, 550, 1700, 550, 1650, 550, 1700, 550, 550, 550, 550, 600, 1650, 550, 550, 600, 500, 600, 550, 550, 550, 600, 500, 600, 1650, 550, 1700, 550, 550, 550, 1700, 550, 1650, 550, 1650, 600};

typedef struct SerialPackage {
   char devicePicker;
   char value;
} Package;

static int irControllerState = 0;

// You should get Auth Token in the Blynk App.
// Go to the Project Settings (nut icon).
char auth[] = "05fd602b578c403ab930882e740445ae";

// Your WiFi credentials.
// Set password to "" for open networks.
char ssid[] = "Leo";
char pass[] = "1234567890";

void setup()
{
  irsend.begin();
  Serial.begin(9600);
  Blynk.begin(auth, ssid, pass);
}

void loop()
{
  Blynk.run();
}

BLYNK_WRITE(V0)
{
  Package pckg;
  pckg.devicePicker = '0';
  pckg.value = param.asInt() + '0';
  
  toSerial(pckg.devicePicker, pckg.value);
}

BLYNK_WRITE(V1)
{
  Package pckg;
  pckg.devicePicker = '1';
  pckg.value = param.asInt() ? '1' : '0';
  
  toSerial(pckg.devicePicker, pckg.value);
}

BLYNK_WRITE(V2)
{
  Package pckg;
  pckg.devicePicker = '2';
  pckg.value = param.asInt() + '0';

  toSerial(pckg.devicePicker, pckg.value);
}

BLYNK_WRITE(V3)
{
  Package pckg;
  pckg.devicePicker = '3';
  pckg.value = param.asInt() ? '1' : '0';

  toSerial(pckg.devicePicker, pckg.value);
}

BLYNK_WRITE(V4)// OFF/ON
{
  if (irControllerState == 0){
    irsend.sendRaw(turnOn, 67, 38);  // Send a raw data capture at 38kHz.
    irControllerState = 1;
  }
  else {
    irsend.sendRaw(turnOff, 67, 38);  // Send a raw data capture at 38kHz.
    irControllerState = 0;
  }
}

BLYNK_WRITE(V5)// RED
{
  irsend.sendRaw(turnRed, 67, 38);  // Send a raw data capture at 38kHz.;
}

BLYNK_WRITE(V6)// GREEN
{
  irsend.sendRaw(turnGreen, 67, 38);  // Send a raw data capture at 38kHz.;
}

BLYNK_WRITE(V7)// BLUE
{
  irsend.sendRaw(turnBlue, 67, 38);  // Send a raw data capture at 38kHz.;
}

BLYNK_WRITE(V8)// ORANGE
{
  irsend.sendRaw(turnOrange, 67, 38);  // Send a raw data capture at 38kHz.;
}

BLYNK_WRITE(V9)// FLASH
{
  irsend.sendRaw(flash, 67, 38);  // Send a raw data capture at 38kHz.;
}

BLYNK_WRITE(V10)// FADE
{
  irsend.sendRaw(fade, 67, 38);  // Send a raw data capture at 38kHz.;
}

void toSerial(char char1, char char2) {
  Serial.print(char1);
  Serial.print(char2);
}



