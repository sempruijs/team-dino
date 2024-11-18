#include <ESP8266WiFi.h>
#include <ESP8266WebServer.h>
#include <SPI.h>
#include <MFRC522.h>

// Wi-Fi Credentials
const char* ssid = "ESP8266_RFID_AP";
const char* password = "password123";

// Define pins for the RC522 module
#define RST_PIN D1  // Reset pin
#define SS_PIN D2   // Slave select pin

MFRC522 mfrc522(SS_PIN, RST_PIN); // Create instance of the RC522
MFRC522::MIFARE_Key key;          // MIFARE key

// Web server instance
ESP8266WebServer server(80);

// Global variable to hold card data
String cardData = "No card detected";

// Setup Function
void setup() {
  Serial.begin(115200); // Start Serial
  SPI.begin();          // Init SPI bus
  mfrc522.PCD_Init();   // Init MFRC522 module

  // Set default key to 0xFF for authentication
  for (byte i = 0; i < 6; i++) {
    key.keyByte[i] = 0xFF;
  }

  // Configure Wi-Fi as an Access Point
  WiFi.softAP(ssid, password);
  Serial.println("Access Point Started");
  Serial.println(WiFi.softAPIP()); // Print the IP address

  // Define web server routes
  server.on("/", handleRoot);
  server.on("/getCardData", handleGetCardData);

  // Start the server
  server.begin();
  Serial.println("HTTP server started");
}

// Loop Function
void loop() {
  server.handleClient(); // Handle client requests
  readCard();            // Continuously check for RFID cards
}

// Function to Read Card Data
void readCard() {
  // Check for a new card
  if (!mfrc522.PICC_IsNewCardPresent() || !mfrc522.PICC_ReadCardSerial()) {
    return;
  }

  // Get UID of the card
  String uid = "";
  for (byte i = 0; i < mfrc522.uid.size; i++) {
    uid += String(mfrc522.uid.uidByte[i], HEX);
    if (i < mfrc522.uid.size - 1) uid += ":";
  }

  // Read data from block 4 (where "Hello RFID!" is stored)
  byte block = 4; // Block where the message is stored
  byte buffer[18]; // Buffer to store the data
  byte size = sizeof(buffer);

  String blockData = "";

  // Authenticate with Key A (default key 0xFFFFFFFFFFFF)
  MFRC522::StatusCode status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Authentication failed with error: ");
    Serial.println(mfrc522.GetStatusCodeName(status)); // Print detailed error
    blockData = "Authentication Failed";
  } else {
    // If authentication succeeds, read the block
    if (mfrc522.MIFARE_Read(block, buffer, &size) == MFRC522::STATUS_OK) {
      // Convert the buffer to a string and discard null bytes
      for (byte i = 0; i < size; i++) {
        if (buffer[i] != 0x00) {  // Ignore NULL bytes
          blockData += (char)buffer[i]; // Convert byte to char
        }
      }
    } else {
      blockData = "Read Failed";
    }
  }

  // Store the card data (display UID and text from the block)
  cardData = "UID: " + uid + " | Card Data: " + blockData;

  // Halt the card and stop encryption
  mfrc522.PICC_HaltA();
  mfrc522.PCD_StopCrypto1();
}

// Root Page (Web Interface)
void handleRoot() {
  String html = R"rawliteral(
  <!DOCTYPE html>
  <html>
  <head>
    <title>ESP8266 RFID Reader</title>
    <script>
      // Function to fetch card data
      function fetchCardData() {
        fetch('/getCardData')
          .then(response => response.text())
          .then(data => {
            document.getElementById('cardData').innerText = data;
          })
          .catch(err => console.error(err));
      }

      // Refresh card data every 2 seconds
      setInterval(fetchCardData, 2000);
    </script>
  </head>
  <body>
    <h1>RFID Card Reader</h1>
    <p id="cardData">Waiting for card data...</p>
  </body>
  </html>
  )rawliteral";

  server.send(200, "text/html", html);
}

// API Endpoint to Get Card Data
void handleGetCardData() {
  server.send(200, "text/plain", cardData); // Sends the UID and the card data (e.g., "Hello RFID")
}
