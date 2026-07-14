import sys
sys.setrecursionlimit(10 ** 6)

def solution(n):
    MOD = 1_000_000_007
    dp = [-1] * (n + 1)
    sum_dp = [-1] * (n + 1)
    
    dp[0] = 1
    if n >= 1: dp[1] = 1
    if n >= 2: dp[2] = 3
    if n >= 3: dp[3] = 10
    
    def recur(x):
        if x < 0:
            return 0
        if dp[x] != -1:
            return dp[x]
            
        res = (recur(x - 1) * 1 + recur(x - 2) * 2 + recur(x - 3) * 5) % MOD
        res = (res + get_sum(x - 4) * 2 + get_sum(x - 5) * 2 + get_sum(x - 6) * 4) % MOD
        
        dp[x] = res
        return dp[x]
        
    def get_sum(x):
        if x < 0:
            return 0
        if sum_dp[x] != -1:
            return sum_dp[x]
            
        sum_dp[x] = (recur(x) + get_sum(x - 3)) % MOD
        return sum_dp[x]
        
    return recur(n)