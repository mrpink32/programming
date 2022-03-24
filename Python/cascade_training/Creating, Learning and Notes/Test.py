import cv2

uno_cascade = cv2.CascadeClassifier("D:/Github/Cascade_Training/Cascades/Custommade/cascade_Uno_green_1.xml")

cap = cv2.VideoCapture(0)

while 1:
    ret, img = cap.read()
    gray = cv2.cvtColor(img, cv2.COLOR_BGR2GRAY)
    cards = uno_cascade.detectMultiScale(gray, 1.5, 5)
    for (x,y,w,h) in cards:
        cv2.rectangle(img,(x,y),(x+w,y+h),(255,0,0),2)

    cv2.imshow("img", img)
    cv2.imshow("gray", gray)
    esc = cv2.waitKey(30) & 0xff
    if esc == 27:
        break

cap.release()
cv2.destroyAllWindows()