"""
GPS経路修正補助システム

A地点からB地点までの距離を出し、m当たりのlatlongを計測する
少数第4位で±5を計測し、しきい値を超えた時点で修正をかける
"""

import pyproj, math

g = pyproj.Geod(ellps="GRS80")

class Correction():
    # 緯度経度からto方角, from方角, 距離(メートル)を取得
    def gps(self, long1, lat1, long2, lat2):

        # g.inv(to_long, to_lat, from_long, from_lat)
        azimuth, bkw_azimuth, distance = g.inv(long1, lat1, long2, lat2)
        return(azimuth, bkw_azimuth, distance)

    # ヘロンの公式
    def heron(self, a, b, c):
        s = 0.5 * (a + b + c)
        large_S = math.sqrt(s * (s - a) * (s - b) * (s - c))
        h = 2 * large_S / a
        return h

    def course(self, h, hdg):
        if h > 1:
            if hdg >= 10:
                return(1)
            elif hdg <= -10:
                return(-1) 
            else:
                return(0)
        else:
            return(0)

    # しきい値計算
    def threshold(self, prev, next, now):
        next_prev = self.gps(next["long"], next["lat"], prev["long"], prev["lat"])
        next_now = self.gps(next["long"], next["lat"], now["long"], now["lat"])
        now_prev = self.gps(now["long"], now["lat"], prev["long"], prev["lat"])

        print(next_now, next_prev, now_prev)

        h = self.heron(next_prev[2], next_now[2], now_prev[2])

        print(h)

        base = next_prev[1] * -1
        hdg = next_now[1] + base

        if hdg > 180:
            hdg = next_now * -1

        return(self.course(h, hdg))

if __name__ == "__main__":

    # 研究所B
    Institute_B = {
        "lat" : 35.62616455678764,
        "long" : 139.34219715172813,
    }

    # 片倉高校
    Katakura_H = {
        "lat" : 35.632018133236116,
        "long" : 139.33117493228036,
    }

    # 八王子みなみ野駅
    Hachi_South = {
        "lat" : 35.63028463240432,
        "long" : 139.34050754938417,
    }

    print(Correction().threshold(Katakura_H, Institute_B, Hachi_South))
