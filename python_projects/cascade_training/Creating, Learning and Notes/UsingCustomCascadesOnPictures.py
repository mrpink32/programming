import numpy as np
import matplotlib as plt
import cv2

Uno_cascade = cv2.CascadeClassifier("D:/Github/Cascade_Training/Cascades/CustomMade/cascade_Uno_3.xml")


while 1:
	img = cv2.imread("D:/Github/Cascade_Training/Tests/Uno (2).png", cv2.IMREAD_COLOR)
	gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
	cards = Uno_cascade.detectMultiScale(gray, 1.5, 3)
	for (x,y,w,h) in cards:
   		cv2.rectangle(img,(x,y),(x+w,y+h),(255,0,0),2)
	cv2.imshow("img", img)
	##cv2.imshow("gray", gray)
	esc = cv2.waitKey(30) & 0xff
	if esc == 27:
		break
cv2.destroyAllWindows()
