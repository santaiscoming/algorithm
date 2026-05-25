def solution(n, w, num):
    num = num - 1
    ans = 0
    
    tr = num // w
    tc = num - tr * w if tr % 2 == 0 else (w - 1) - (num - tr * w)
    
    for v in range(n):
        r = v // w
        c = v - r * w if r % 2 == 0 else (w - 1) - (v - r * w)
        
        if tc == c :
            ans += 1
            
    return ans - tr
        
    