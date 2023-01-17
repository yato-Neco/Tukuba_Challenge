import pyproj, math

g = pyproj.Geod(ellps="GRS80")

def gps(lat1, long1, lat2, long2):
    # g.inv(to_long, to_lat, from_long, from_lat)
    azimuth, bkw_azimuth, distance = g.inv(long1, lat1, long2, lat2)
    # 返り値 =
    return(azimuth, bkw_azimuth, distance)

def heron(a, b, c):
    s = 0.5 * (a + b + c)
    return math.sqrt(s * (s - a) * (s - b) * (s - c))

next_prev = gps(35.62616455678764, 139.34219715172813, 35.632018133236116, 139.33117493228036) # 研B → みなみ野

now_prev = gps(35.63028463240432, 139.34050754938417, 35.632018133236116, 139.33117493228036) # みなみ野 → 片倉高校

next_now = gps(35.62616455678764, 139.34219715172813, 35.63028463240432, 139.34050754938417) # 研B → 片倉高校

print(next_prev, now_prev, next_now)
S = heron(next_prev[2], now_prev[2], next_now[2]) 
h = 2 * S / next_prev[2]
print(S)
print(h)

# if next_prev[0] >= 0:
tmp = next_prev[0] * -1

to_azmith = next_now[1] + tmp
print(to_azmith)