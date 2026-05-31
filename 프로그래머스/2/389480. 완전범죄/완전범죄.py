def solution(info, n, m):
    INF = float('inf')
    l = len(info)
    
    memo = [[-1] * m for _ in range(l)]
    
    def recur(depth, b_acc):
        if b_acc >= m:
            return INF
            
        if depth == l:
            return 0
            
        if memo[depth][b_acc] != -1:
            return memo[depth][b_acc]
            
        a_cst, b_cst = info[depth]
        
        pick_a = recur(depth + 1, b_acc) + a_cst
        pick_b = recur(depth + 1, b_acc + b_cst)
        
        memo[depth][b_acc] = min(pick_a, pick_b)
        
        return memo[depth][b_acc]
        
    res = recur(0, 0)
    
    return res if res < n else -1