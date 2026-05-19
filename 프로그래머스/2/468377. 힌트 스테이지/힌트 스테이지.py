def solution(cost, hint):
    INF = float('inf')    
    n = len(cost)
    answer = INF
    
    def dfs(depth, acc, bundle):
        nonlocal answer
        cnt = n - 1 if bundle[depth] > n - 1 else bundle[depth]
        
        if depth == n - 1:
            curr = cost[depth][cnt]
            answer = min(answer, acc + curr)
            return
        
        
        dfs(depth + 1, acc + cost[depth][cnt], bundle)
        
        new_bundle = bundle[:]
        for v in hint[depth][1:]:
            new_bundle[v - 1] += 1
        dfs(depth + 1, acc + cost[depth][cnt] + hint[depth][0], new_bundle)
        
    
    dfs(1, cost[0][0], [0] * n)
    
    bundle = [0] * n
    for v in hint[0][1:]:
        bundle[v - 1] += 1
    dfs(1, cost[0][0] + hint[0][0], bundle)
    
    return answer