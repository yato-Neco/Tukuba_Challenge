from tkinter.messagebox import NO
from typing import List, Tuple, Dict, Set, Optional

class GPS:
    
    LonLat1 = None
    LonLat2 = None
    LonLat3 = None
    LonLat4 = None

    def __init__(self, LonLat1:Tuple[float,float], LonLat2:Tuple[float,float] = (None,None), LonLat3:Tuple[float,float] = (None,None), LonLat4:Tuple[float,float] = (None,None),):
        self.LonLat1 = LonLat1
        self.LonLat2 = LonLat2
        self.LonLat3 = LonLat3
        self.LonLat4 = LonLat4

    def is_into_x4(self,LonLat:Tuple[float,float]) -> bool:
        pass
        
        

    def is_into_x2(self, LonLat:Tuple[float,float]) -> bool:

        if self.LonLat1[0] < LonLat[0] < self.LonLat4[0] and self.LonLat1[1] < LonLat[1] < self.LonLat4[1]:
            return True
        else:
            return False

    def println(self) -> None:
        print('%s %s %s %s' % (self.LonLat1, self.LonLat2, self.LonLat3, self.LonLat4))


tmp = GPS((35.627490,139.339816),(35.627608,139.340224),(35.627774,139.339650),(35.627939,139.339977))
print(tmp.is_into_x2((35.627791,139.339956)))