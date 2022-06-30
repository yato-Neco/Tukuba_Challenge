from GPS import GPS
import serial_asyncio  # 自己責任らしい
import asyncio
import time

now = []


class Output(asyncio.Protocol):
    def connection_made(self, transport):
        self.transport = transport
        print('port opened', transport)
        transport.serial.rts = False

    def data_received(self, data):
        global now
        pdata = None
        #['2022/06/27 06:04:11.000673', ' numSat:14', ' Fix', ' Lat=35.627744', ' Lon=139.339773']
        try:
            print(data.strip().decode('UTF-8').split(","))
            pdata = data.strip().decode('UTF-8').split(",")[3:5]
        except Exception as e:
            print(e)
        now = pdata

    def connection_lost(self, exc):
        print('port closed')
        asyncio.get_event_loop().stop()


async def gps_test():
    print("NAV Start!!")

    tmp = GPS(
        point_list=[
            (35.627148, 139.340235), (35.626668, 139.340600), (35.626254, 139.341007), (35.625778, 139.341501)])

    while True:

        if len(now) == 2:
            Lon = float(now[1])
            Lat = float(now[0])

            tmp.nav((Lat, Lon))

        await asyncio.sleep(0.1)


loop = asyncio.get_event_loop()
coro = serial_asyncio.create_serial_connection(
    loop, Output, 'COM3', baudrate=115200)
results = loop.run_until_complete(asyncio.gather(
    gps_test(),
    coro
))
loop.run_forever()
loop.close()
