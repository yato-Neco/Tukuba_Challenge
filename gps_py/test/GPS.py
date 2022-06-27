from typing import List, Tuple, Dict, Set, Optional
from pyproj import Geod
import asyncio


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

    def is_into_r(self, now_LatLon: Tuple[float, float], r: float) -> bool:
        """
        GPSの引数LatLon1を中心点としたほぼ半径2rにいるか判定
        ＊円なのかも怪しい....
        """

        if (self.LatLon1[0] - r) < now_LatLon[0] < (self.LatLon1[0] + r) and (self.LatLon1[1] - r) < now_LatLon[1] < (self.LatLon1[1] + r):
            return True
        else:
            return False

    def zaimuth_return(self, now_LatLon: Tuple[float, float], LatLon: Tuple[float, float] = None) -> Dict["azimuth", "back_azimuth"]:
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

    def nav(self, now_LatLon: Tuple[float, float], flag:bool = False) -> None:

        if flag:
            self.point_list.pop(0)

        print(self.point_list)
        print(self.zaimuth_return(now_LatLon, self.point_list[0]))




        """
        Example

        [(0.0,1.0),(2.0,3.0),(4.0,5.0)]


        0 -> 1 -> 2 -> 3


        """
        pass

    

    def println(self) -> None:
        print('%s %s %s %s' %
              (self.LatLon1, self.LatLon2, self.LatLon3, self.LatLon4))


