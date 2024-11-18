#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SPI.h>
#include <MFRC522.h>

#define SS_PIN D2  // SPI Slave Select pin for MFRC522
#define RST_PIN D1 // Reset pin for MFRC522
MFRC522 mfrc522(SS_PIN, RST_PIN);

ESP8266WebServer server(80); // Web server on port 80

MFRC522::MIFARE_Key key; // Define the MIFARE key globally

String cardData = "Waiting for RFID card...";
String inputData = ""; // Data to write to the card

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

  // Define the root webpage route
  server.on("/", []() {
    String html = "<html><head><title>RFID Reader</title>";
    html += "<script>function updateCardData() {";
    html += "fetch('/card').then(response => response.text()).then(data => {";
    html += "document.getElementById('cardData').innerHTML = data; });";
    html += "setTimeout(updateCardData, 1000); }"; // Refresh every second
    html += "</script></head><body onload='updateCardData()'>";
    html += "<h1>RFID Reader & Writer</h1>";
    html += "<p>Scan an RFID card to see its details below:</p>";
    html += "<h2>Card Data:</h2>";
    html += "<pre id='cardData'>Waiting for data...</pre>";
    html += "<h2>Write to Card:</h2>";
    html += "<form action='/write' method='POST'>";
    html += "Text to Write: <input type='text' name='text'><br>";
    html += "<input type='submit' value='Write to Card'>";
    html += "</form>";
    html += "</body></html>";
    server.send(200, "text/html", html); // Send the webpage to the client
  });

  // Define the /card route to send card data
  server.on("/card", []() {
    server.send(200, "text/plain", cardData); // Send RFID data as plain text
  });

  // Define the /write route to write data to the card
  server.on("/write", HTTP_POST, []() {
    if (server.hasArg("text")) {
      inputData = server.arg("text");
      cardData = "Waiting to write: " + inputData;
    }
    server.sendHeader("Location", "/");
    server.send(303); // Redirect to the main page
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

  // Read stored data on the card
  String cardText = readCardData();
  cardData += "\nStored Text: " + cardText;

  // If inputData is available, write it to the card
  if (!inputData.isEmpty()) {
    if (writeCardData(inputData)) {
      cardData += "\nData Written Successfully!";
    } else {
      cardData += "\nFailed to Write Data!";
    }
    inputData = ""; // Clear input after writing
  }

  Serial.println(cardData); // Print to Serial Monitor for debugging

  // Halt the card
  mfrc522.PICC_HaltA();
}

bool writeCardData(String data) {
  MFRC522::StatusCode status;
  byte block = 1;       // Block to write data
  byte buffer[16] = {}; // Initialize an empty buffer

  // Ensure the data fits in 16 bytes
  data.getBytes(buffer, sizeof(buffer));

  // Authenticate with the card
  status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Authentication failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
    return false;
  }

  // Write the data to the card
  status = mfrc522.MIFARE_Write(block, buffer, 16);
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Write failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
    return false;
  }

  Serial.println("Write successful");
  return true;
}

String readCardData() {
  MFRC522::StatusCode status;
  byte block = 1;        // Block to read data from
  byte buffer[18] = {};  // Buffer to hold the read data
  byte size = sizeof(buffer);

  // Authenticate with the card
  status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Authentication failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
    return "Read Failed";
  }

  // Read the data from the card
  status = mfrc522.MIFARE_Read(block, buffer, &size);
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Read failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
    return "Read Failed";
  }

  // Convert buffer to a string and return
  String result = "";
  for (byte i = 0; i < 16; i++) {
    if (buffer[i] != 0) result += (char)buffer[i];
  }
  return result;
}
