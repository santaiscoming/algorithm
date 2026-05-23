def solution(depth, money, excavate):
    w = len(depth)
    
    depth = [0] + depth
    memo = [[-1] * (w + 2) for _ in range(w + 2)]
    nxt = [[0] * (w + 2) for _ in range(w + 2)]
    
    def recur(l, r):
        if l > r:
            return 0
        
        if memo[l][r] != -1:
            return memo[l][r]
        
        min_cost = float('inf')
        selec_k = l
        
        for k in range(l, r + 1):
            cur_cost = depth[k] + max(recur(l, k - 1), recur(k + 1, r))
            
            if cur_cost < min_cost:
                min_cost = cur_cost
                selec_k = k
                
        nxt[l][r] = selec_k
        memo[l][r] = min_cost
        
        return min_cost

    recur(1, w)
    
    l, r = 1, w
    while l <= r:
        k = nxt[l][r]
        res = excavate(k)
        
        if res == 0:
            return k
        elif res == -1:
            r = k - 1
        else:
            l = k + 1
            
    return -1