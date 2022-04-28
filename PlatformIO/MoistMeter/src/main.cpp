#include <Arduino.h>
#include <WiFi.h>
#include <Client.h>

unsigned int g_sensorPin = 34;
int g_sensorValue;
char serverAddress[] = "192.168.50.52";
char ssid[] = "Paulick";
char password[] = "mikkelfp";
const int g_PORT = 9000;
WiFiClient client;
int connectionToServer = 0;


void setup() {
  // put your setup code here, to run once:  
  Serial.begin(115200);
  Serial.println("Hello World!");
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.println("Conneecting to wifi...");
  }
  Serial.print("WiFi connected with IP: ");
  Serial.println(WiFi.localIP());
}

void loop() {
  // put your main code here, to run repeatedly:
  while (!connectionToServer)
  {
    connectionToServer = client.connect(serverAddress, g_PORT);
    delay(500);
    if (!connectionToServer)
    {
      Serial.println("Retrying...");
    }
  }
  g_sensorValue = analogRead(g_sensorPin);
  Serial.printf("Sensor Value: ");
  Serial.println(g_sensorValue);
  client.println(g_sensorValue);
  // Send id to server
  client.println("Close");
  delay(1000);
  client.flush();
  client.stop();
  delay(5000);
}

