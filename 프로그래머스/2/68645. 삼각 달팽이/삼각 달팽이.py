def solution(n):
    t = [[0] * i for i in range(1, n + 1)]
    r, c = -1, 0
    num = 1
    
    for i in range(n):
        for _ in range(i, n):
            if i % 3 == 0:
                r += 1
            elif i % 3 == 1:
                c += 1
            elif i % 3 == 2:
                r -= 1
                c -= 1
                
            t[r][c] = num
            num += 1
            
    return [v for r in t for v in r]