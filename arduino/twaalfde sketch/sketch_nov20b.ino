#include <SPI.h>
#include <MFRC522.h>
#include <ESP8266WiFi.h>
#include <ESP8266HTTPClient.h>
#include <WiFiClient.h>

// Wi-Fi credentials
const char* ssid = "Pikmin"; // Replace with your Wi-Fi SSID
const char* password = "SSBU0709RBX"; // Replace with your Wi-Fi password

// Server endpoint
const char* serverURL = "http://108.61.188.109:3030/check_card/"; // Replace with your server URL

// Define pins for RC522 and ESP8266
#define RST_PIN D1  // Reset pin
#define SS_PIN D2   // Slave Select (SDA) pin

// Create an instance of the MFRC522 class
MFRC522 mfrc522(SS_PIN, RST_PIN);

// Create a WiFiClient object
WiFiClient client;

void setup() {
  // Initialize serial communication
  Serial.begin(115200);
  while (!Serial); // Wait for serial port to initialize
  Serial.println("Initializing...");

  // Initialize SPI bus and MFRC522 module
  SPI.begin();
  mfrc522.PCD_Init();

  // Connect to Wi-Fi
  Serial.print("Connecting to Wi-Fi");
  WiFi.begin(ssid, password);
  while (WiFi.status() != WL_CONNECTED) {
    delay(500);
    Serial.print(".");
  }
  Serial.println("\nConnected to Wi-Fi!");
}

void loop() {
  // Check if a new card is present
  if (!mfrc522.PICC_IsNewCardPresent()) {
    return;
  }

  // Check if a card has been read
  if (!mfrc522.PICC_ReadCardSerial()) {
    return;
  }

  // Construct the card ID from UID
  String cardID = "";
  for (byte i = 0; i < mfrc522.uid.size; i++) {
    cardID += String(mfrc522.uid.uidByte[i], HEX);
  }

  Serial.println("Card Detected. Sending ID to server...");
  Serial.print("Card ID: ");
  Serial.println(cardID);

  // Make the HTTP GET request
  if (WiFi.status() == WL_CONNECTED) {
    HTTPClient http;
    String fullURL = String(serverURL) + cardID; // Append card ID to the URL

    // Use the WiFiClient object with http.begin
    Serial.println(fullURL);
    http.begin(client, fullURL);

    int httpCode = http.GET(); // Send the GET request
    if (httpCode > 0) {
      // Get the response
      String payload = http.getString();
      Serial.print("Server Response: ");
      Serial.println(payload);

      // Check server response
      if (payload == "true") {
        Serial.println("Access Granted!");
      } else {
        Serial.println("Access Denied!");
      }
    } else {
      Serial.print("HTTP Request failed. Error code: ");
      Serial.println(httpCode);
    }
    http.end(); // Close the connection
  } else {
    Serial.println("Wi-Fi Disconnected!");
  }

  // Halt the PICC to prevent repeated reads
  mfrc522.PICC_HaltA();
  delay(1000); // Small delay to avoid spamming the server
}
