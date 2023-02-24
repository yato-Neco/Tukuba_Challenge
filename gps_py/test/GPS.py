from typing import List, Tuple, Dict, Set, Optional

import scipy as sp
from pyproj import Geod
import asyncio

#GPSのclass
class GPS:

    LatLon1 = None
    LatLon2 = None
    LatLon3 = None
    LatLon4 = None
    point_list = []



    def __init__(self, LatLon1: Tuple[float, float] = None, LatLon2: Tuple[float, float] = (None, None), LatLon3: Tuple[float, float] = (None, None), LatLon4: Tuple[float, float] = (None, None), point_list = []):
        self.point_list = point_list
        self.LatLon1 = LatLon1
        self.LatLon2 = LatLon2
        self.LatLon3 = LatLon3
        self.LatLon4 = LatLon4

    def is_into_x4(self, now_LatLon: Tuple[float, float]) -> bool:
        """
        4点のBOXの中にいるか判定

        """
        pass

    def is_into_x2(self, now_LatLon: Tuple[float, float]) -> bool:
        """
        非推奨
        2点からなるBoxの中にいるか判定
        """

        if self.LatLon1[0] < now_LatLon[0] < self.LatLon4[0] and self.LatLon1[1] < now_LatLon[1] < self.LatLon4[1]:
            return True
        else:
            return False

    def is_into_r(self, now_LatLon: Tuple[float, float], LatLon1: Tuple[float, float] = None,  r: float  = 0.0) -> bool:
        """
        GPSの引数LatLon1を中心点としたほぼ半径2rにいるか判定
        ＊円なのかも怪しい....
        """

        if LatLon1 == None:

            if (self.LatLon1[0] - r) < now_LatLon[0] < (self.LatLon1[0] + r) and (self.LatLon1[1] - r) < now_LatLon[1] < (self.LatLon1[1] + r):
                return True
            else:
                return False
        else:
            if (LatLon1[0] - r) < now_LatLon[0] < (LatLon1[0] + r) and (LatLon1[1] - r) < now_LatLon[1] < (LatLon1[1] + r):
                return True
            else:
                return False


    def azimuth_return(self, now_LatLon: Tuple[float, float], LatLon: Tuple[float, float] = None) -> Dict["azimuth", "back_azimuth"]:
        """
        緯度経度で角度を求める
        """

        g = Geod(ellps='WGS84')

        if LatLon != None:
            result = g.inv(now_LatLon[1], now_LatLon[0],
                           LatLon[1], LatLon[0])
            azimuth = result[0]
            back_azimuth = result[1]

            return {"azimuth": azimuth, "back_azimuth": back_azimuth}

        else:

            result = g.inv(now_LatLon[1], now_LatLon[0],
                           self.LatLon1[1], self.LatLon1[0])
            azimuth = result[0]
            back_azimuth = result[1]

            return {"azimuth": azimuth, "back_azimuth": back_azimuth}
    

    def spped(self, now_LatLon: Tuple[float, float], LatLon: Tuple[float, float] = None,) -> float:
        g = Geod(ellps='WGS84')
        time :float = 3.0
        result = g.inv(now_LatLon[1], now_LatLon[0], LatLon[1], LatLon[0])
        distance = result[3]


        speed = distance / time 


        return speed

    def nav(self, now_LatLon: Tuple[float, float],) -> None:

        flag = self.is_into_r(now_LatLon,self.point_list[0],0.0001)

        if flag:
            flag = False
            self.point_list.pop(0)
            print("Ok")
            print(self.azimuth_return(now_LatLon, self.point_list[0]))
            print(self.point_list)

        
        if len(self.point_list) == 0:
            print("GG")


    

    def println(self) -> None:
        print('%s %s %s %s' %
              (self.LatLon1, self.LatLon2, self.LatLon3, self.LatLon4))






g = Geod(ellps='WGS84')

pos_a = (35.629514,139.904730)
pos_b = (35.629614,139.904830)

result = g.inv(pos_a[1], pos_a[0],
                           pos_b[1], pos_b[0])
azimuth = result[0]
print(azimuth)
#39.22883123445416