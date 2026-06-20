import math

def solution(k, d):
    ans = 0
    
    for x in range(0, d + 1, k):
        y = math.sqrt(d**2 - x**2)
        ans += int(y // k) + 1
        
    return ans