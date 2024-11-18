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

// Global variables to hold card data and text to write
String textToWrite = "";
String cardUID = "";

void readCard() {
  String response = "No card detected.";
  
  if (mfrc522.PICC_IsNewCardPresent() && mfrc522.PICC_ReadCardSerial()) {
    cardUID = "";
    for (byte i = 0; i < mfrc522.uid.size; i++) {
      cardUID += String(mfrc522.uid.uidByte[i], HEX);
      if (i < mfrc522.uid.size - 1) cardUID += ":";
    }
    response = "Card UID: " + cardUID;
  }

  server.send(200, "text/plain", response);  // Send the UID back to the client
}

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
  server.on("/submitText", HTTP_POST, handleSubmitText); // Handle text submission
  server.on("/readCard", HTTP_GET, handleReadCard); // Handle reading card
  server.on("/writeCard", HTTP_POST, handleWriteCard); // Handle writing to card

  // Start the server
  server.begin();
  Serial.println("HTTP server started");
}

// Loop Function
void loop() {
  server.handleClient(); // Handle client requests
  readCard();            // Continuously check for RFID cards
}

// Function to Read Card Data (Triggered by Read Card button)
void handleReadCard() {
  String response = "No card detected.";
  
  if (mfrc522.PICC_IsNewCardPresent() && mfrc522.PICC_ReadCardSerial()) {
    cardUID = "";
    for (byte i = 0; i < mfrc522.uid.size; i++) {
      cardUID += String(mfrc522.uid.uidByte[i], HEX);
      if (i < mfrc522.uid.size - 1) cardUID += ":";
    }
    response = "Card UID: " + cardUID;
  }

  server.send(200, "text/plain", response);  // Send the UID back to the client
}

// Function to Write Text to the RFID Card (Triggered by Write Card button)
void handleWriteCard() {
  String message = "No card detected.";

  // Wait for a card and write text when detected
  if (mfrc522.PICC_IsNewCardPresent() && mfrc522.PICC_ReadCardSerial()) {
    byte block = 4; // Block where we want to write the text
    byte data[16];
    textToWrite.getBytes(data, 16);  // Convert the string to a byte array

    // Authenticate with Key A (default key 0xFFFFFFFFFFFF)
    MFRC522::StatusCode status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
    if (status != MFRC522::STATUS_OK) {
      message = "Authentication failed: " + String(mfrc522.GetStatusCodeName(status));
      server.send(200, "text/plain", message);
      return;
    }

    // Write the data to the block
    status = mfrc522.MIFARE_Write(block, data, 16);
    if (status == MFRC522::STATUS_OK) {
      message = "Data written to card successfully!";
      textToWrite = "";  // Clear the text after writing
    } else {
      message = "Write failed: " + String(mfrc522.GetStatusCodeName(status));
    }

    server.send(200, "text/plain", message); // Send the response back to the client
  } else {
    message = "No card detected.";
    server.send(200, "text/plain", message);
  }
}

// Root Page (Web Interface)
void handleRoot() {
  String html = R"rawliteral(
  <!DOCTYPE html>
  <html>
  <head>
    <title>ESP8266 RFID Reader</title>
    <script>
      // Function to submit the text for writing to the card
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

      // Function to read the card when the "Read Card" button is pressed
      function readCard() {
        fetch("/readCard")
          .then(response => response.text())
          .then(data => {
            document.getElementById('cardData').innerText = data;
          })
          .catch(err => console.error(err));
      }

      // Function to write the text to the card when the "Write Card" button is pressed
      function writeCard() {
        fetch("/writeCard", {
          method: "POST"
        })
        .then(response => response.text())
        .then(data => {
          document.getElementById('writeResponse').innerText = data;
        })
        .catch(err => console.error(err));
      }
    </script>
  </head>
  <body>
    <h1>RFID Card Reader and Writer</h1>

    <!-- Section for writing text to the card -->
    <p>Enter text to write to the RFID card:</p>
    <input type="text" id="textToWrite" />
    <button onclick="submitText()">Submit Text</button>

    <!-- Display response for writing text -->
    <p id="response"></p>
    <p id="writeResponse"></p>

    <!-- Section for reading card -->
    <button onclick="readCard()">Read Card</button>
    <p id="cardData">Card data will appear here</p>

    <!-- Section for writing text to the card -->
    <button onclick="writeCard()">Write to Card</button>
  </body>
  </html>
  )rawliteral";

  server.send(200, "text/html", html);
}

// Handle form submission from the webpage (receive the text to write to card)
void handleSubmitText() {
  String message = "No text received.";

  if (server.hasArg("plain")) {
    String body = server.arg("plain");
    int start = body.indexOf("\"text\":\"") + 8;
    int end = body.indexOf("\"}", start);
    textToWrite = body.substring(start, end);
    message = "Text received: " + textToWrite;
  }

  server.send(200, "text/plain", message);
}
