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

// Global variable to hold the text to write to the card
String textToWrite = "";

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
  server.on("/", handleRoot);              // Home page with form
  server.on("/submitText", HTTP_POST, handleSubmitText); // Handle form submission
  server.on("/getCardData", handleGetCardData);  // Endpoint for fetching card data

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

  // Only write to the card if text is available to write
  if (textToWrite != "") {
    byte block = 4; // Block where we want to write the text
    byte data[16];
    textToWrite.getBytes(data, 16);  // Convert the string to a byte array

    // Authenticate with Key A (default key 0xFFFFFFFFFFFF)
    MFRC522::StatusCode status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
    if (status != MFRC522::STATUS_OK) {
      Serial.print("Authentication failed: ");
      Serial.println(mfrc522.GetStatusCodeName(status));
      return;
    }

    // Write the data to the block
    status = mfrc522.MIFARE_Write(block, data, 16);
    if (status == MFRC522::STATUS_OK) {
      Serial.println("Data written to card successfully!");
      textToWrite = "";  // Clear the text after writing to the card
    } else {
      Serial.print("Write failed: ");
      Serial.println(mfrc522.GetStatusCodeName(status));
    }
  }

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
      // Function to submit the text from the input form
      function submitText() {
        var text = document.getElementById("textToWrite").value;
        fetch("/submitText", {
          method: "POST",
          body: JSON.stringify({ text: text }),
          headers: { "Content-Type": "application/json" }
        })
        .then(response => response.text())
        .then(data => {
          document.getElementById('response').innerText = data;
        })
        .catch(err => console.error(err));
      }
    </script>
  </head>
  <body>
    <h1>RFID Card Writer</h1>
    <p>Enter text to write to the RFID card:</p>
    <input type="text" id="textToWrite" />
    <button onclick="submitText()">Submit Text</button>
    <p id="response"></p>
  </body>
  </html>
  )rawliteral";

  server.send(200, "text/html", html);
}

// Handle form submission from the webpage
void handleSubmitText() {
  String message = "No text received";

  // Read the incoming JSON data
  if (server.hasArg("plain")) {
    String body = server.arg("plain");
    // Extract text from the JSON body
    int start = body.indexOf("\"text\":\"") + 8;
    int end = body.indexOf("\"}", start);
    textToWrite = body.substring(start, end);
    message = "Text received: " + textToWrite;
  }

  server.send(200, "text/plain", message);
}

// API Endpoint to Get Card Data
void handleGetCardData() {
  String cardData = "No card detected"; // Default response
  if (mfrc522.PICC_IsNewCardPresent()) {
    // Read UID of the card
    String uid = "";
    if (mfrc522.PICC_ReadCardSerial()) {
      for (byte i = 0; i < mfrc522.uid.size; i++) {
        uid += String(mfrc522.uid.uidByte[i], HEX);
        if (i < mfrc522.uid.size - 1) uid += ":";
      }
      cardData = "UID: " + uid;
    }
  }
  server.send(200, "text/plain", cardData);
}
