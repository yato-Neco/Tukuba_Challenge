import RPI.GPIO as GPIO
import sys
from time import sleep, time
from pynput import keyboard


def setup():
    # モーターPIN設定
    gpio_left = [22, 23]
    gpio_right = [24, 25]

    GPIO.setmode(GPIO.BCM)

    GPIO.setup(gpio_left[0], GPIO.OUT)
    GPIO.setup(gpio_left[1], GPIO.OUT)
    GPIO.setup(gpio_right[0], GPIO.OUT)
    GPIO.setup(gpio_right[1], GPIO.OUT)

    main(gpio_left, gpio_right)

def main(left, right):
    p1 = GPIO.PWM(left[0], 0)
    p2 = GPIO.PWM(left[1], 0)
    p3 = GPIO.PWM(right[0], 0)
    p4 = GPIO.PWM(right[0], 0)

    p1.start(0)
    p2.start(0)
    p3.start(0)
    p4.start(0)

    print("ready")
    
    def on_press(key):
        duty_1 = 0  # 正
        duty_2 = 0  # 負

        if key == "w":
            if duty_2 > 0:
                p2.ChangeDutyCycle(duty_2 - 1)
                p4.ChangeDutyCycle(duty_2 - 1)
                print(duty_1, duty_2)
            else:
                p1.ChangeDutyCycle(duty_1 + 1)
                p3.ChangeDutyCycle(duty_1 + 1)
                print(duty_1, duty_2)

        if key == "s":
            if duty_1 > 0:
                p1.ChangeDutyCycle(duty_1 - 1)
                p3.ChangeDutyCycle(duty_1 - 1)
                print(duty_1, duty_2)
            else:
                p2.ChangeDutyCycle(duty_2 + 1)
                p4.ChangeDutyCycle(duty_2 + 1)
                print(duty_1, duty_2)

        if key == "a":
            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(20)
            p3.ChangeDutyCycle(20)
            p4.ChangeDutyCycle(0)

        if key == "d":
            p1.ChangeDutyCycle(20)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(20)

        if key == keyboard.Key.space:
            duty_1, duty_2 = 0
            p1.ChangeDutyCycle(0)
            p2.ChangeDutyCycle(0)
            p3.ChangeDutyCycle(0)
            p4.ChangeDutyCycle(0)

    def on_release(key):
        if key == keyboard.Key.esc:
            GPIO.clean()
            keyboard.Listener.stop()
            sys.exit()


    with keyboard.Listener(
            on_press=on_press,
            on_release=on_release) as listener:
        listener.join()


