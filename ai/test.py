import torch
import torch.nn as nn
import torch.nn.functional as F
import os
import numpy as np
import matplotlib.pyplot as plt
from tkinter import Tk, Button, Label, filedialog
from PIL import Image
import torchvision.transforms as transforms
import cv2
import pytesseract
import numpy as np

# Specify the path to the tesseract executable (for Windows)
pytesseract.pytesseract.tesseract_cmd = r'C:/Program Files/Tesseract-OCR/tesseract.exe'

# Step 4: One-hot encode the text labels for input to the model
def one_hot_encode_label(label, max_length=10, num_classes=38):
    # Padding with "_" if the label is shorter than 8 characters
    label = label.ljust(max_length, '_')
    
    encoded = torch.zeros(max_length, num_classes, device=device)
    for i, char in enumerate(label[:max_length]):
        if char.isalpha():
            idx = ord(char.upper()) - ord('A')
        elif char.isdigit():
            idx = ord(char) - ord('0') + 26
        elif char == '_':  # Treat padding as a special character
            idx = 36  # Adding an extra index for padding
        elif char == '-':  # Treat padding as a special character
            idx = 37  # Adding an extra index for padding
        encoded[i, idx] = 1
    return encoded.flatten()  # Flatten to a 1D tensor

# Define the model architecture
class LicensePlateClassifier(nn.Module):
    def __init__(self, input_size=380, hidden_size=25, output_size=3):  # output_size = 3 for correct, incorrect, incomplete
        super(LicensePlateClassifier, self).__init__()
        self.fc1 = nn.Linear(input_size, hidden_size)
        self.fc2 = nn.Linear(hidden_size, hidden_size)
        self.fc3 = nn.Linear(hidden_size, output_size)  # 3 outputs

    def forward(self, x):
        x = torch.relu(self.fc1(x))
        x = torch.relu(self.fc2(x))
        x = self.fc3(x)  # Raw scores for each class
        return x

    def load(self, file_name="license_plate_classifier.pth"):
        self.load_state_dict(torch.load(file_name, map_location=device))
        self.eval()

# Define the model architecture
class UpscalingNet(nn.Module):
    def __init__(self):
        super(UpscalingNet, self).__init__()
        self.conv1 = nn.Conv2d(3, 192, kernel_size=3, padding=1)
        self.conv2 = nn.Conv2d(192, 1024, kernel_size=3, padding=1)
        self.conv3 = nn.ConvTranspose2d(1024, 3, kernel_size=4, stride=2, padding=1)

    def forward(self, x):
        x = F.relu(self.conv1(x))
        x = F.relu(self.conv2(x))
        x = self.conv3(x)
        return x

    def load(self, file_name="image_upscaling_model_32x32_x2_v5.pth"):
        self.load_state_dict(torch.load(file_name, weights_only=False))
        self.eval()

class ImageUpscalerApp:
    def __init__(self, master):
        self.master = master
        self.master.title("Image Upscaler and License Plate Detector")

        # Initialize the label for the buttons
        self.label = Label(master, text="Select an image")
        self.label.pack()

        self.select_button = Button(master, text="Select Image", command=self.load_image)
        self.select_button.pack()

        self.detect_button = Button(master, text="Detect License Plate", command=self.detect_license_plate)
        self.detect_button.pack()

        self.upscale_button = Button(master, text="Upscale and Re-Detect", command=self.upscale_and_detect, state="disabled")
        self.upscale_button.pack()

        self.save_button = Button(master, text="Save Upscaled Image", command=self.save_image, state="disabled")
        self.save_button.pack()

        # Load the upscaling model
        self.upscaling_model = UpscalingNet().to(device)
        self.upscaling_model.load()

        # Load the license plate detection model
        self.plate_classifier = LicensePlateClassifier().to(device)
        self.plate_classifier.load('license_plate_classifier.pth')  # Load the saved model

        self.transform = transforms.Compose([
            transforms.Resize((16, 16)),
            transforms.ToTensor(),
        ])

        self.image_path = None
        self.original_image = None
        self.current_image = None  # To store the current (upscaled) image
        self.upscale_attempts = 0  # Track number of upscales allowed per image

    def load_image(self):
        """Load an image selected by the user."""
        self.image_path = filedialog.askopenfilename(title="Select an Image",
                                                     filetypes=[("Image Files", "*.jpg;*.jpeg;*.png")])
        if self.image_path:
            self.original_image = Image.open(self.image_path).convert("RGB")
            self.current_image = self.original_image  # Set the current image to the original image
            self.label.config(text=os.path.basename(self.image_path))
            self.upscale_button.config(state="normal")  # Enable the upscale button
            self.upscale_attempts = 0  # Reset upscale attempts

    def detect_license_plate(self, image=None):
        """Detect license plate in the provided image."""
        if image is None and self.current_image:
            image = np.array(self.current_image)[:, :, ::-1].copy()  # Convert to OpenCV format (BGR)

        if image is None:
            print("No image loaded for detection.")
            return

        plate_text = self.detect_plate(image)
        if plate_text:
            print(f"Detected License Plate: {plate_text}")
            # Now classify the plate text using the classifier model
            plate_text = plate_text.strip()
            if len(plate_text) == 0:
                print("No text detected in plate.")
                self.upscale_attempts += 1
                self.upscale_and_detect()
            else:
                encoded_plate = one_hot_encode_label(plate_text).unsqueeze(0).to(device)  # Convert text to one-hot encoding
                with torch.no_grad():
                    output = self.plate_classifier(encoded_plate)
                    _, predicted = torch.max(output, 1)
                    classes = ["Correct", "Incorrect", "Incomplete"]
                    plate_class = classes[predicted.item()]
                    print(f"License Plate Class: {plate_class}")
                    if plate_class == "Incorrect" or plate_class == "Incomplete":
                        self.upscale_attempts += 1
                        self.upscale_and_detect()
        else:
            print("No license plates found or detected text unclear.")
            self.upscale_attempts += 1
            self.upscale_and_detect()

    def upscale_and_detect(self):
        """Upscale the current image and attempt to detect the license plate on the upscaled image."""
        if self.current_image:
            if self.upscale_attempts < 4:  # Allow up to 2 upscales
                upscaled_blocks = []
                width, height = self.current_image.size
                new_width = (width // 16) * 16
                new_height = (height // 16) * 16
                resized_image = self.current_image.resize((new_width, new_height), Image.BICUBIC)
       
                # Process image in 16x16 blocks and upscale each block
                for i in range(0, new_height, 16):
                    for j in range(0, new_width, 16):
                        block = resized_image.crop((j, i, j + 16, i + 16))
                        img_tensor = self.transform(block).unsqueeze(0).to(device)
                        with torch.no_grad():
                            upscaled_tensor = self.upscaling_model(img_tensor)  # Use self.upscaling_model instead of self.model
                        upscaled_block = upscaled_tensor.squeeze(0).cpu().clamp(0, 1)
                        upscaled_image = transforms.ToPILImage()(upscaled_block)
                        upscaled_blocks.append(upscaled_image)
       
                self.current_image = self.stitch_blocks(upscaled_blocks, new_width // 16, new_height // 16)  # Set current_image to the newly upscaled image
       
                # Convert upscaled image to OpenCV format and run detection again
                upscaled_cv_image = np.array(self.current_image)[:, :, ::-1].copy()
                plate_text = self.detect_plate(upscaled_cv_image)
                if plate_text:
                    print(f"Detected License Plate (Upscaled): {plate_text}")
                    encoded_plate = one_hot_encode_label(plate_text).unsqueeze(0).to(device)  # Convert text to one-hot encoding
                    with torch.no_grad():
                        output = self.plate_classifier(encoded_plate)
                        _, predicted = torch.max(output, 1)
                        classes = ["Correct", "Incorrect", "Incomplete"]
                        plate_class = classes[predicted.item()]
                        print(f"License Plate Class: {plate_class}")
                        if plate_class == "Incorrect" or plate_class == "Incomplete":
                            self.upscale_attempts += 1
                            self.upscale_and_detect()
                else:
                    print("No license plates found after upscaling or detected text unclear.")
                    self.upscale_attempts += 1
                    self.upscale_and_detect()
       
                self.save_button.config(state="normal")  # Enable the save button
       
                # Increment upscale attempt counter and disable button if limit is reached
                self.upscale_attempts += 1
                if self.upscale_attempts >= 3:
                    self.upscale_button.config(state="disabled")
            else:
                print("Upscaling limit reached. Further upscaling would degrade image quality.")

    def stitch_blocks(self, blocks, cols, rows):
        """Stitch 32x32 blocks together to form the final upscaled image."""
        block_width, block_height = 32, 32  # Upscaled block size
        final_image = Image.new("RGB", (cols * block_width, rows * block_height))
        for row in range(rows):
            for col in range(cols):
                final_image.paste(blocks[row * cols + col], (col * block_width, row * block_height))
        return final_image

    def detect_plate(self, image):
        """Use OpenCV and Tesseract to detect license plates in the image."""
        gray = cv2.cvtColor(image, cv2.COLOR_BGR2GRAY)
        plate_cascade = cv2.CascadeClassifier(cv2.data.haarcascades + 'haarcascade_russian_plate_number.xml')
        plates = plate_cascade.detectMultiScale(gray, scaleFactor=1.1, minNeighbors=5, minSize=(30, 30))
        if len(plates) == 0:
            return None
        for (x, y, w, h) in plates:
            plate_region = image[y:y+h, x:x+w]
            plate_gray = cv2.cvtColor(plate_region, cv2.COLOR_BGR2GRAY)
            plate_gray = cv2.bilateralFilter(plate_gray, 11, 17, 17)
            plate_gray = cv2.threshold(plate_gray, 150, 255, cv2.THRESH_BINARY)[1]
            plate_text = pytesseract.image_to_string(plate_gray, config='--psm 8 -c tessedit_char_whitelist=ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-')
            plate_text = plate_text.strip()  # Remove leading/trailing whitespaces
            plate_text = plate_text[:10]  # Ensure it's only 8 characters
            cv2.imshow("License Plate", plate_region)
            return plate_text.strip()

    def show_images(self, original, low_res, upscaled):
        """Display original, low-res resized, and upscaled images using Matplotlib."""
        plt.figure(figsize=(18, 6))
        plt.subplot(1, 3, 1)
        plt.title("Original Image")
        plt.imshow(original)
        plt.axis('off')

        plt.subplot(1, 3, 2)
        plt.title("Resized Image")
        plt.imshow(low_res)
        plt.axis('off')

        plt.subplot(1, 3, 3)
        plt.title("Upscaled Image")
        plt.imshow(upscaled)
        plt.axis('off')

        plt.show()

    def save_image(self):
        """Save the upscaled image to disk."""
        if self.current_image:
            save_path = filedialog.asksaveasfilename(defaultextension=".png",
                                                     filetypes=[("PNG files", "*.png"),
                                                                ("JPEG files", "*.jpg;*.jpeg"),
                                                                ("All files", "*.*")])
            if save_path:
                self.current_image.save(save_path)
                print(f"Upscaled image saved as: {save_path}")

if __name__ == '__main__':
    device = torch.device('cuda' if torch.cuda.is_available() else 'cpu')
    root = Tk()
    app = ImageUpscalerApp(root)
    root.mainloop()
