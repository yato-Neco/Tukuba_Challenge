import serial
import time
import binascii


ser = serial.Serial("COM8", 9600)
t = (0x1F00FFFF).to_bytes(4, byteorder="big")
print(t)

while True:
    time.sleep(0.1)
    ser.write(t)
    result = ser.read_all()
    if result != b'':
        print(result)