def solution(k, dungeons):
    n = len(dungeons)
    ans = recur(0, [False] * n, k, dungeons)
    
    return ans

def recur(depth, visited, sp, dungeons):
    n = len(dungeons)
    if depth > n:
        return 0

    best = 0
    for i in range(n):
        bound, s = dungeons[i]
        if (not visited[i] and
            sp >= bound and
            sp >= s):
            visited[i] = True
            sp -= s
            best = max(best, recur(depth + 1, visited, sp, dungeons) + 1) 
            visited[i] = False
            sp += s

    return best                