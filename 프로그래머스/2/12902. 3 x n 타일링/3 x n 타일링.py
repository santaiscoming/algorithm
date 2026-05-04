import sys
sys.setrecursionlimit(100000)

def solution(n):
    mod = 1_000_000_007
    memo = [-1] * (n + 1)
    
    def recur(n):
        if n == 0:
            return 1
        if n == 2:
            return 3
            
        if memo[n] != -1:
            return memo[n]
            
        res = (recur(n - 2) * 4 - recur(n - 4)) % mod
        memo[n] = res
        
        return res
        
    return recur(n)