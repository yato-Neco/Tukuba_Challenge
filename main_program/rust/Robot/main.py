from ast import While
from calendar import c
from itertools import count
import serial
import time
import binascii

ser = serial.Serial("COM6", 9600)


num = 1


# f = list_byte(num)


s = bytes(b'aa')
#s = list(s)
print(s.hex())

t = (0x1F00FFFF).to_bytes(4, byteorder="big")
f = (100).to_bytes(4, byteorder="big")

t = list(t)
f = list(f)
print(t)


# print(list((0.2).to_bytes(4, byteorder="big")))


ser.write(t)

while True:
    #ser.write(t)
	
    time.sleep(0.1)
    
    result = ser.read_all()
    if result != b'':
        #print(result)
        tmp = (int.from_bytes(result, 'big'))
        print(((tmp & 0xF0000000) >> 28),((tmp & 0x00F00000) >> 20),((tmp & 0x000F0000) >> 16))
