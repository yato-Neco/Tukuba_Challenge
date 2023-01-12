"""
GPS経路修正補助システム

A地点からB地点までの距離を出し、m当たりのlatlongを計測する
少数第4位で±5を計測し、しきい値を超えた時点で修正をかける
"""

import pyproj, math

g = pyproj.Geod(ellipsis="GRS80")


class Correction():
    
    # 緯度経度からto方角, from方角, 距離(メートル)を取得
    def gps(self, lat1, long1, lat2, long2):

        # g.inv(to_long, to_lat, from_long, from_lat)
        azimuth, bkw_azimuth, distance = g.inv(long1, lat1, long2, lat2)
        return(azimuth, bkw_azimuth, distance)

    def threshold(self):
