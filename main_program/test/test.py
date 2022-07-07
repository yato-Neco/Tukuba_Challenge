from typing import List, Tuple, Dict, Set, Optional
import time
import concurrent.futures

#import RPi.GPIO as GPIO


class Robot:


    executor = None

    def __init__(self):
        """
        初期化
        """

        self.executor = concurrent.futures.ThreadPoolExecutor(max_workers=1)
    
    def run(self):
        """
        走行時
        """

        tmp = Motor()
        self.executor.submit(tmp.controler)


        while True:

            tmp.flag_chang(1)
            print("main")
            time.sleep(10.0)
            tmp.flag_chang(5)
        




        

    def senser():
        """
        センサー類
        """

        
        def lider():
            pass




class Motor:

    moterPins = [[0,0],[0,0]]

    flag = 0

    def __init__(self, moterPins: List = [[24,25],[0,0]]):
        """
        moterPinsにGPIO pin
        """
        self.moterPins = moterPins
        #executor = concurrent.futures.ThreadPoolExecutor(max_workers=2)


    def init(self) -> None:
        """
        初期化
        forward(), backward(), backward(), right(), left() 使うときに必要
        """

        """
        GPIO.setwarnings(False)
        GPIO.setmode(GPIO.BCM)
        GPIO.setup(self.moterPins,GPIO.OUT)
        """

    def controler(self):
        tmp = self.flag
        while True:
            if tmp == 0:
                self.stop()
            elif tmp == 1:
                self.forward()
            elif tmp == 2:
                self.backward()
            elif tmp == 3:
                self.right()
            elif tmp == 4:
                self.left()
            elif tmp == 5:
                self.manualMode()
            else:
                self.stop()
                break



    def flag_chang(self,flag:int) -> None:
        """
        マルチスレッド処理終了のフラグ。
        82 117 115 116 227 129 160 227 129 163 227 129 159 227 130 137 227 130 130 227 129 163 227 129 168 227 129 190 227 129 151 227 129 171 230 155 184 227 129 145 227 130 139 46
        """
        self.flag = flag


    def forward(self) -> None:
        """
        前進
        """

        """
        GPIO.output(self.moterPins[0][1],GPIO.HIGH)
        GPIO.output(self.moterPins[0][0],GPIO.LOW)

        GPIO.output(self.moterPins[1][1],GPIO.HIGH)
        GPIO.output(self.moterPins[1][0],GPIO.LOW)
        """


        print("forward")
        time.sleep(1.0)

        return None
        
        


    def backward(self) -> None:
        """
        後進
        """
        
        """
        GPIO.output(self.moterPins[0][1],GPIO.LOW)
        GPIO.output(self.moterPins[0][0],GPIO.HIGH)

        GPIO.output(self.moterPins[1][1],GPIO.LOW)
        GPIO.output(self.moterPins[1][0],GPIO.HIGH)
        """


    def right(self) -> None:
        pass

    def left(self) -> None:
        pass


    def stop(self) -> None:
        """
        停止
        """

        """
        GPIO.output(self.moterPins[0][0],GPIO.HIGH)
        GPIO.output(self.moterPins[0][1],GPIO.HIGH)

        GPIO.output(self.moterPins[0][0],GPIO.HIGH)
        GPIO.output(self.moterPins[0][1],GPIO.HIGH)
        """

    
    def manualMode(self, moterPin: List = [24,25]) -> None:

        """
        片輪だけ動かすしたりする為の関数
        """

        
        print("manualMode")
        time.sleep(1.0)

        return None

        """
        GPIO.output(moterPin[0],GPIO.HIGH)
        GPIO.output(moterPin[1],GPIO.HIGH)
        """
        
