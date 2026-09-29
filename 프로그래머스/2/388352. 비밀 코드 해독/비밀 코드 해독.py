from itertools import combinations

def solution(n, q, ans):
    m = len(ans)
    answer = 0
    
    for secret in combinations(range(1, n + 1), 5):
        candi = [False] * m
        for i in range(m):
            if len(set(secret) & set(q[i])) == ans[i]:
                candi[i] = True
                
        if all(candi):
            answer += 1
    
    return answer