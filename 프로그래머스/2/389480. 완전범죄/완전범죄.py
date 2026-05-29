def solution(info, n, m):
    INF = float('inf')
    
    dp = [INF] * m
    dp[0] = 0
    
    for a_cst, b_cst in info:
        n_dp = [INF] * m
        
        for cur_b in range(m):
            if dp[cur_b] == INF:
                continue
                
            nxt_a = dp[cur_b] + a_cst
            if nxt_a < n:
                n_dp[cur_b] = min(n_dp[cur_b], nxt_a)
                
            nxt_b = cur_b + b_cst
            if nxt_b < m:
                n_dp[nxt_b] = min(n_dp[nxt_b], dp[cur_b])
                
        dp = n_dp
        
    res = min(dp)
    
    return res if res < INF else -1