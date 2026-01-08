def solution(n, tops):
    MOD = 10007
    a = 1
    b = 0
    
    for i in range(n):
        if tops[i] == 1:
            new_a = (a * 3 + b * 2) % MOD
            new_b = (a * 1 + b * 1) % MOD
        else:
            new_a = (a * 2 + b * 1) % MOD
            new_b = (a * 1 + b * 1) % MOD
            
        a, b = new_a, new_b
    
    return (a + b) % MOD