def solution(order):
    n = len(order)
    
    ans = 0
    s = []
    t = 0
    
    for v in range(1, n + 1):
        if order[t] == v:
            t += 1
            ans += 1
        else:
            s.append(v)
            
        while s and order[t] == s[-1]:
            s.pop()
            ans += 1
            t += 1
        
    return ans