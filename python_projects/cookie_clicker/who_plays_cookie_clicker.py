import ctypes, random
from msilib.schema import Upgrade
import os, cv2, time
import numpy as np
from threading import *
import pyautogui


def get_cookie_pos():
    screen = np.array(pyautogui.screenshot())
    cv2.imwrite('screen.png', screen)
    img_rgb = cv2.imread('screen.png')
    img_gray = cv2.cvtColor(img_rgb, cv2.COLOR_BGR2GRAY)
    template = cv2.imread('cookie.png', 0)
    w, h = template.shape[::-1]
    res = cv2.matchTemplate(img_gray,template,cv2.TM_CCOEFF_NORMED)
    threshold = 0.5
    loc = np.where( res >= threshold)
    for pt in zip(*loc[::-1]):
        pos = int(pt[0] + w/2), int(pt[1] + h/2)
    return pos
    #debugging
        #cv2.rectangle(img_rgb, pt, (pt[0] + w, pt[1] + h), (0,0,255), 2)
    #cv2.imwrite('res.png',img_rgb)


def get_upgrade_pos():
    img_rgb = cv2.imread('screen.png')
    img_gray = cv2.cvtColor(img_rgb, cv2.COLOR_BGR2GRAY)
    template = cv2.imread('hand.png', 0)
    w, h = template.shape[::-1]
    res = cv2.matchTemplate(img_gray,template,cv2.TM_CCOEFF_NORMED)
    threshold = 0.7
    loc = np.where( res >= threshold)
    for pt in zip(*loc[::-1]):
        pos = int(pt[0] + w/2), int(pt[1] + h/2)
    return pos, h
    #debugging
        #cv2.rectangle(img_rgb, pt, (pt[0] + w, pt[1] + h), (0,0,255), 2)
    #cv2.imwrite('res.png',img_rgb)


def gcookie_search():
    # to do use open cv object match to find golden cookies
    pass


def click(x, y):
    print(x, y)
    ctypes.windll.user32.SetCursorPos(x, y)
    ctypes.windll.user32.mouse_event(LEFT_CLICK_DOWN)
    ctypes.windll.user32.mouse_event(LEFT_CLICK_UP)
    time.sleep(0.02)


def main():
    while True:
        for i in range(0, 500):
            click(COOKIE_POS[0], COOKIE_POS[1])
            if random.randint(0, 75) == random.randint(0, 75):
                click(int(SCREEN_WIDTH/8*7-300), int(SCREEN_HEIGHT/4-240))
        for j in range(0, 7):
            click(UPGRADE_POS[0], UPGRADE_POS[1] + UPGRADE_HEIGHT*j)
            #click(int(SCREEN_WIDTH/8*7), int(SCREEN_HEIGHT/4+200))
            #click(int(SCREEN_WIDTH/8*7), int(SCREEN_HEIGHT/4+400))
    # while True:
    #     if ():
    #         break
    #gcookie_thread = Thread.start(target=gcookie_search())


if __name__ == "__main__":
    time.sleep(5)
    SCREEN_WIDTH, SCREEN_HEIGHT = ctypes.windll.user32.GetSystemMetrics(0), ctypes.windll.user32.GetSystemMetrics(1)
    COOKIE_POS = get_cookie_pos()
    UPGRADE_POS, UPGRADE_HEIGHT = get_upgrade_pos()
    LEFT_CLICK_DOWN = 2 #"0x0002"
    LEFT_CLICK_UP = 4 #"0x0004"
    ESC = "0x1B"
    main()
