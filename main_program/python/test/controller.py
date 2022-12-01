import RPi.GPIO as GPIO
import sys
from time import sleep, time
from pynput import keyboard


def setup():
    # モーターPIN設定
    gpio_left = [23, 22]
    gpio_right = [24, 25]

    GPIO.setmode(GPIO.BCM)

    GPIO.setup(gpio_left[0], GPIO.OUT)
    GPIO.setup(gpio_left[1], GPIO.OUT)
    GPIO.setup(gpio_right[0], GPIO.OUT)
    GPIO.setup(gpio_right[1], GPIO.OUT)

    main(gpio_left, gpio_right)

def main(left, right):
    freq = 50

    p1 = GPIO.PWM(left[0], freq)
    p2 = GPIO.PWM(left[1], freq)
    p3 = GPIO.PWM(right[0], freq)
    p4 = GPIO.PWM(right[1], freq)

    p1.start(0)
    p2.start(0)
    p3.start(0)
    p4.start(0)

    print("ready")
    
    duty_1 = 10  # 正
    duty_2 = 5  # 負

    while True:
        key = input("W:前進 S:後退 A:左旋回 D:右旋回 F:停止 K:終了\n")

        if key == "w":
            # if duty_2 > 0:
            #     p2.ChangeDutyCycle(duty_1)
            #     p4.ChangeDutyCycle(duty_1)
            # else:
            #     p1.ChangeDutyCycle(duty_1)
            #     p3.ChangeDutyCycle(duty_1)
            
            p1.ChangeDutyCycle(duty_1)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(duty_1)
            p4.ChangeDutyCycle(0)

        if key == "s":
            # if duty_1 > 0:
            #     p1.ChangeDutyCycle(duty_1)
            #     p3.ChangeDutyCycle(duty_1)
            # else:
            #     p2.ChangeDutyCycle(duty_1)
            #     p4.ChangeDutyCycle(duty_1)

            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(duty_1)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(duty_1)


        if key == "a":
            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(duty_2)
            p3.ChangeDutyCycle(duty_2)
            p4.ChangeDutyCycle(0)

        if key == "d":
            p1.ChangeDutyCycle(duty_2)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(duty_2)

        if key == "f":
            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(0)

        if key == "k":
            duty_1, duty_2 = 0
            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(0)
            GPIO.clean()
            sys.exit()
            

setup()