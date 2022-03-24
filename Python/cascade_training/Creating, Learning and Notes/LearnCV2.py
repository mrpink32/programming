import cv2
import numpy as np
import matplotlib.pyplot as plt

# 0 for greyscale	1 for color	-1 for unchanged
img = cv2.imread("C:/Users/mikke/Desktop/Neural_Network/Pictures/RPP/img500.png", cv2.IMREAD_COLOR)
cv2.line(img, (0, 0), (1920, 1080), (0, 255, 0))
cv2.rectangle(img, (640, 360), (1280, 720), (255, 0, 0))
cv2.circle(img, (960, 540), 50, (0, 0, 255))
cv2.imshow("image",img)
cv2.waitKey(0)
cv2.destroyAllWindows()

