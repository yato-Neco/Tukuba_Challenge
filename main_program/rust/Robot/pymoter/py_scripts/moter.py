
#import RPi.GPIO as GPIO

class Moter:

    gpio_left = [23, 22]
    gpio_right = [24, 25]

    moter_left = []
    moter_right = []

    freq = 50


    def __init__(self):
        #GPIO.setmode(GPIO.BCM)
        #GPIO.setup(self.gpio_left[0], GPIO.OUT)
        #GPIO.setup(self.gpio_left[1], GPIO.OUT)
        #GPIO.setup(self.gpio_right[0], GPIO.OUT)
        #GPIO.setup(self.gpio_right[1], GPIO.OUT)


        #p1 = GPIO.PWM(self.gpio_left[0], self.freq)
        #p2 = GPIO.PWM(self.gpio_left[1], self.freq)
        #p3 = GPIO.PWM(self.gpio_right[0], self.freq)
        #p4 = GPIO.PWM(self.gpio_right[1], self.freq)

        self.moter_left = [p1,p2]
        self.moter_right = [p3,p4]

        pass

    def left(self,r_duty,l_duty):
        self.moter_left[0].ChangeDutyCycle(0)
        self.moter_left[1].ChangeDutyCycle(r_duty)
        self.moter_right[0].ChangeDutyCycle(l_duty)
        self.moter_right[1].ChangeDutyCycle(0)
        

    
    def right(self,r_duty,l_duty):
        self.moter_left[0].ChangeDutyCycle(r_duty)
        self.moter_left[1].ChangeDutyCycle(0)
        self.moter_right[0].ChangeDutyCycle(0)
        self.moter_right[1].ChangeDutyCycle(l_duty)
        

    def front(self,r_duty,l_duty):
        self.moter_left[0].ChangeDutyCycle(r_duty)
        self.moter_left[1].ChangeDutyCycle(0)
        self.moter_right[0].ChangeDutyCycle(l_duty)
        self.moter_right[1].ChangeDutyCycle(0)
        

    def back(self,r_duty,l_duty):
        self.moter_left[0].ChangeDutyCycle(0)
        self.moter_left[1].ChangeDutyCycle(r_duty)
        self.moter_right[0].ChangeDutyCycle(0)
        self.moter_right[1].ChangeDutyCycle(l_duty)
        

    def stop(self):
        self.moter_left[0].ChangeDutyCycle(0)
        self.moter_left[1].ChangeDutyCycle(0)
        self.moter_right[0].ChangeDutyCycle(0)
        self.moter_right[1].ChangeDutyCycle(0)



