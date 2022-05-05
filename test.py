import pyautogui

import time
i = 387
time.sleep(2)

while i < 10000:
    pyautogui.write(str(i))
    pyautogui.press('enter')
    time.sleep(0.8)
    i+=1