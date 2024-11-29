#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SPI.h>
#include <MFRC522.h>

#define SS_PIN D2  // SPI Slave Select pin for MFRC522
#define RST_PIN D1 // Reset pin for MFRC522
MFRC522 mfrc522(SS_PIN, RST_PIN);

ESP8266WebServer server(80); // Web server on port 80

String cardData = "Waiting for RFID card...";

void setup() {
  Serial.begin(115200); // Start serial communication for debugging
  SPI.begin();          // Initialize SPI bus
  mfrc522.PCD_Init();   // Initialize the RFID reader
  Serial.println("RFID Reader Initialized");

  // Set up ESP8266 as an access point
  WiFi.softAP("ESP8266_RFID", "12345678");
  Serial.println("WiFi AP Started");
  Serial.print("IP Address: ");
  Serial.println(WiFi.softAPIP()); // Display the ESP8266 IP address

  // Define the root route (webpage)
  server.on("/", []() {
    String html = "<html><head><title>RFID Reader</title>";
    html += "<script>function updateCardData() {";
    html += "fetch('/card').then(response => response.text()).then(data => {";
    html += "document.getElementById('cardData').innerHTML = data; });";
    html += "setTimeout(updateCardData, 1000); }"; // Refresh every second
    html += "</script></head><body onload='updateCardData()'>";
    html += "<h1>RFID Reader</h1>";
    html += "<p>Scan an RFID card to see its details below:</p>";
    html += "<h2>Card Data:</h2>";
    html += "<pre id='cardData'>Waiting for data...</pre>";
    html += "</body></html>";
    server.send(200, "text/html", html); // Send the webpage to the client
  });

  // Define the /card route to send card data
  server.on("/card", []() {
    server.send(200, "text/plain", cardData); // Send RFID data as plain text
  });

  server.begin(); // Start the web server
  Serial.println("Web server started");
}

void loop() {
  server.handleClient(); // Handle HTTP requests

  // Check if a new RFID card is present
  if (!mfrc522.PICC_IsNewCardPresent() || !mfrc522.PICC_ReadCardSerial())
    return;

  // Read and format the UID
  cardData = "UID: ";
  for (byte i = 0; i < mfrc522.uid.size; i++) {
    cardData += String(mfrc522.uid.uidByte[i], HEX);
    if (i < mfrc522.uid.size - 1)
      cardData += ":";
  }

  // Append card type information
  cardData += "\nCard Type: ";
  cardData += mfrc522.PICC_GetTypeName(mfrc522.PICC_GetType(mfrc522.uid.sak));

  Serial.println(cardData); // Print to Serial Monitor for debugging

  // Halt the card
  mfrc522.PICC_HaltA();
}
