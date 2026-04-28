import math


def lcm(arr):
    lcm = arr[0]
    for i in range(1, len(arr)):
        lcm = (lcm * arr[i]) // math.gcd(lcm, arr[i])
    return lcm

def solution(signals):
    cycles = [g + y + r for g, y, r in signals]
    mt = lcm(cycles)
    
    for t in range(1, mt + 1):
        all_yellow = True
        
        for i in range(len(signals)):
            g, y, r = signals[i]
            c = cycles[i]
            rem = t  % c
                
            if not (g < rem <= g + y):
                all_yellow = False
                break 
            
        if all_yellow:
            return t
    
    return -1