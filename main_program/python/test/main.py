import RPi.GPIO as GPIO
from time import sleep

# Pins for Motor Driver Inputs 
Motor1A = 25
 
def setup():
    GPIO.setwarnings(False)
    GPIO.setmode(GPIO.BCM)              # GPIO Numbering
    GPIO.setup(Motor1A,GPIO.OUT)  # All pins as Outputs
 
def loop():
    # Going forwards
    GPIO.output(Motor1A,GPIO.HIGH)
    print("Going forwards")
 
    sleep(5)
    # Going backwards
    GPIO.output(Motor1A,GPIO.HIGH)
    print("Going backwards")
 
    sleep(5)
    # Stop
    GPIO.output(Motor1A,GPIO.LOW)
    print("Stop")

def destroy():  
    GPIO.cleanup()

if __name__ == '__main__':     # Program start from here
    setup()
    try:
            loop()
    except KeyboardInterrupt:
        destroy()