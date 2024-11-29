#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SPI.h>
#include <MFRC522.h>

#define SS_PIN D2  // SPI Slave Select pin
#define RST_PIN D1 // Reset pin for MFRC522
MFRC522 mfrc522(SS_PIN, RST_PIN);

// Create a web server on port 80
ESP8266WebServer server(80);

// Variable to hold the UID and Card Type
String cardData = "Waiting for RFID card...";

void setup() {
  Serial.begin(115200);
  SPI.begin();           // Initialize SPI bus
  mfrc522.PCD_Init();    // Initialize MFRC522
  Serial.println("RFID Reader Initialized");

  // Start WiFi as an Access Point
  WiFi.softAP("ESP8266_RFID", "12345678");
  Serial.println("WiFi AP Started");
  Serial.print("IP Address: ");
  Serial.println(WiFi.softAPIP());

  // Define the root webpage handler
  server.on("/", []() {
    String html = "<html><head><title>RFID Reader</title></head><body>";
    html += "<h1>RFID Reader</h1>";
    html += "<p>Scan an RFID card to see its details.</p>";
    html += "<h2>Card Data:</h2>";
    html += "<pre>" + cardData + "</pre>";
    html += "</body></html>";
    server.send(200, "text/html", html);
  });

  // Start the server
  server.begin();
  Serial.println("Web server started");
}

void loop() {
  // Handle web server requests
  server.handleClient();

  // Check if a new card is present and read its data
  if (!mfrc522.PICC_IsNewCardPresent() || !mfrc522.PICC_ReadCardSerial())
    return;

  // Format the UID into a string
  cardData = "UID: ";
  for (byte i = 0; i < mfrc522.uid.size; i++) {
    cardData += String(mfrc522.uid.uidByte[i], HEX);
    if (i < mfrc522.uid.size - 1)
      cardData += ":";
  }

  // Add card type information
  cardData += "\nCard Type: ";
  cardData += mfrc522.PICC_GetTypeName(mfrc522.PICC_GetType(mfrc522.uid.sak));

  Serial.println(cardData); // Print to Serial Monitor for debugging

  // Halt the current card
  mfrc522.PICC_HaltA();
}
