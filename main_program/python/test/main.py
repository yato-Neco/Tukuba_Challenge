from tkinter import Variable
import RPi.GPIO as GPIO
from time import sleep, time

Motor1A = 24
Motor1B = 25

# Duty比 速度の調整
duty = 50

# 周波数 PWM周波の調整
freq = 50

GPIO.setmode(GPIO.BCM)

GPIO.setup(Motor1A, GPIO.OUT)
GPIO.setup(Motor1B, GPIO.OUT)
# GPIO.setup(Motor2A, GPIO.OUT)
# GPIO.setup(Motor2B, GPIO.OUT)

p1 = GPIO.PWM(Motor1A, freq)
p2 = GPIO.PWM(Motor1B, freq)
# p3 = GPIO.PWM(Motor2A, freq)
# p4 = GPIO.PWM(Motor2B, freq)

p1.start(0)
p2.start(0)
# p3.start(0)
# p4.start(0)
    
"""
モーターコントロール関数
ブレーキは100, 100
解放は0, 0
正転はn, 0 (n = 0 ~ 100)
負転は0, n (n = 0 ~ 100) 
"""
def motor_controller(left_motor_p1, left_motor_p2):
    p1.ChangeDutyCycle(left_motor_p1)
    p2.ChangeDutyCycle(left_motor_p2)

def motor_clean():
    GPIO.cleanup()

# 以下テスト用
def motor_high():
    p1.ChangeDutyCycle(100)
    p2.ChangeDutyCycle(0)

def motor_medium():
    p1.ChangeDutyCycle(50)
    p2.ChangeDutyCycle(0)

def motor_low():
    p1.ChangeDutyCycle(0)
    p2.ChangeDutyCycle(0)

def motor_back():
    p1.ChangeDutyCycle(0)
    p2.ChangeDutyCycle(10)

def motor_test():
    motor_high()
    print("high")
    sleep(3)
    motor_medium()
    print("mid")
    sleep(3)
    # motor_low()
    # print("low")
    # sleep(3)
    motor_back()
    print("back")
    sleep(0.5)

    print("test end")
    GPIO.cleanup()


"""
いらないもの集
"""

# setup 周波数50Hz
# GPIO.setup(Motor1A, GPIO.OUT)
# GPIO.setup(Motor1B, GPIO.OUT)
# GPIO.setup(Motor2A, GPIO.OUT)
# GPIO.setup(Motor2B, GPIO.OUT)

# p1 = GPIO.PWM(Motor1A, freq)
# p3 = GPIO.PWM(Motor2A, freq)
# p4 = GPIO.PWM(Motor2B, freq)

# p1.start(0)
# p2.start(0)
# p3.start(0)
# p4.start(0)

# GPIO 初期設定 
# Motor1A = 24
# Motor1B = 25
# Motor2A = 32
# Motor2B = 36