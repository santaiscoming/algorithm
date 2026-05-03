def get_time(lv, d, t):
    ret = 0
    
    for i in range(len(d)):
        if d[i] <= lv:
            ret += t[i]
        else:            
            ret += (d[i] - lv) * (t[i] + t[i-1]) + t[i]
            
    return ret

def solution(diffs, times, limit):
    l, r = 1, max(diffs)
    ans = r
    
    while l <= r:
        m = (l + r) // 2  
        
        if get_time(m, diffs, times) <= limit:
            ans = m
            r = m - 1
        else:
            l = m + 1
            
    return ans