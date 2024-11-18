#include <SPI.h>
#include <MFRC522.h>

#define RST_PIN D1  // Reset pin
#define SS_PIN D2   // Slave select pin

MFRC522 mfrc522(SS_PIN, RST_PIN);  // Create instance of the MFRC522

MFRC522::MIFARE_Key key;  // Key for authentication

void setup() {
  Serial.begin(115200);
  SPI.begin();  // Initialize SPI bus
  mfrc522.PCD_Init();  // Initialize MFRC522

  // Set the default key (0xFF for Key A)
  for (byte i = 0; i < 6; i++) {
    key.keyByte[i] = 0xFF;
  }

  Serial.println("Place your RFID card near the reader...");
}

void loop() {
  // Look for a new card
  if (!mfrc522.PICC_IsNewCardPresent()) {
    return;
  }

  // Select one of the cards
  if (!mfrc522.PICC_ReadCardSerial()) {
    return;
  }

  // Show the UID of the card
  String uid = "";
  for (byte i = 0; i < mfrc522.uid.size; i++) {
    uid += String(mfrc522.uid.uidByte[i], HEX);
    if (i < mfrc522.uid.size - 1) uid += ":";
  }
  Serial.println("Card detected!");
  Serial.print("UID: ");
  Serial.println(uid);

  // Block to write to
  byte block = 4;
  String dataString = "Hello, RFID!";  // Data to write (up to 16 bytes)

  // Pad the data to be exactly 16 bytes
  while (dataString.length() < 16) {
    dataString += " ";  // Add spaces until the string is 16 bytes long
  }

  byte data[16];
  dataString.getBytes(data, 16);  // Convert the string to a byte array

  // Authenticate using Key A
  MFRC522::StatusCode status = mfrc522.PCD_Authenticate(MFRC522::PICC_CMD_MF_AUTH_KEY_A, block, &key, &(mfrc522.uid));
  
  if (status != MFRC522::STATUS_OK) {
    Serial.print("Authentication failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
    return;
  }

  // Write the data to the block
  status = mfrc522.MIFARE_Write(block, data, 16);  // Write 16 bytes to the block
  
  if (status == MFRC522::STATUS_OK) {
    Serial.println("Data written to card successfully!");
  } else {
    Serial.print("Write failed: ");
    Serial.println(mfrc522.GetStatusCodeName(status));
  }

  mfrc522.PICC_HaltA();  // Halt the PICC (card)
  mfrc522.PCD_StopCrypto1();  // Stop encryption
  delay(1000);  // Wait for the next card
}
