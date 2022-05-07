#include <Arduino.h>
#include <Preferences.h>
#include <WiFi.h>
#include <vector>

#define SENSOR 35
#define SENSORPOWER 25

unsigned int g_sensorValue;
char ssid[] = "Grim";
char password[] = "mikkelfp";
const int g_PORT = 9000;
WiFiServer server(g_PORT);

// Assign output variables to GPIO pins
const int output26 = 26;
const int output27 = 27;

TaskHandle_t Task1;
TaskHandle_t Task2;

void readSensor(void * pvParameters)
{
  for (;;)
  {
    digitalWrite(SENSORPOWER, HIGH);
    delay(5000);
    g_sensorValue = analogRead(SENSOR);
    Serial.printf("Sensor Value: ");
    Serial.println(g_sensorValue);
    digitalWrite(SENSORPOWER, LOW);
    delay(10000);
  }
}

void stuff(void * pvParameters)
{
  for (;;)
  {
    WiFiClient client = server.available();

    if (client)
    {
      Serial.println("New client!");
      while (client.connected())
      {
        client.print(g_sensorValue);
        delay(5000);
      }
      client.stop();
      Serial.println("Client disconnected: " + String(client.connected()));
    }
    delay(1000);
  }
}

void setup()
{
  // put your setup code here, to run once:
  Serial.begin(115200);
  // Initialize the output variables as outputs
  pinMode(output26, OUTPUT);
  pinMode(output27, OUTPUT);
  pinMode(SENSORPOWER, OUTPUT);
  digitalWrite(SENSORPOWER, LOW);
  // Connect to Wi-Fi network with SSID and password
  Serial.print("Setting AP (Access Point)…");
  // Remove the password parameter, if you want the AP (Access Point) to be open
  WiFi.softAP(ssid, password, 7);
  IPAddress IP = WiFi.softAPIP();
  Serial.print("AP IP address: ");
  Serial.println(IP);
  server.begin();
  Serial.println("Setup done!");
  xTaskCreatePinnedToCore(
                    stuff,   /* Task function. */
                    "Task1",     /* name of task. */
                    10000,       /* Stack size of task */
                    NULL,        /* parameter of the task */
                    1,           /* priority of the task */
                    &Task1,      /* Task handle to keep track of created task */
                    0);          /* pin task to core 1 */
  xTaskCreatePinnedToCore(
                    readSensor,   /* Task function. */
                    "Task2",     /* name of task. */
                    10000,       /* Stack size of task */
                    NULL,        /* parameter of the task */
                    1,           /* priority of the task */
                    &Task2,      /* Task handle to keep track of created task */
                    1);          /* pin task to core 1 */
}

void loop()
{
  
}
