def solution(players, m, k):
    cnt = 0
    cur = 0
    exp = [0] * 48
    
    for t in range(24):
        cur -= exp[t]
        req = players[t] // m
        
        if req > cur:
            add = req - cur
            cnt += add
            cur += add
            exp[t + k] += add
            
    return cnt