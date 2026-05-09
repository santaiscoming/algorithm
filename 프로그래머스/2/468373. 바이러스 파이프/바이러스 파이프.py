from collections import deque

def solution(n, infection, edges, k):
    ans = 0
    graph = [[] for _ in range(n + 1)]
    for x, y, typ in edges:
        graph[x].append((y, typ))
        graph[y].append((x, typ))
    
    
    def dfs(q, visited, depth, cnt, pipe):
        nonlocal ans
        
        if depth == k or not q:
            ans = max(ans, cnt)
            return
        
        new_q = deque()
        while q:
            u = q.popleft()
            new_q.append(u)
            
            for v, typ in graph[u]:
                if not visited[v] and pipe == typ:
                    new_q.append(u)
                    q.append(v)
                    cnt += 1
                    visited[v] = True
        
        for next in [1, 2, 3]:
            dfs(deque(new_q), visited[:], depth + 1, cnt, next)
            
    for pipe in [1, 2, 3]:
        q = deque([infection])
        visited = [False] * (n + 1)
        visited[infection] = True
        
        dfs(deque(list(q)), visited, 0, 1, pipe)
    
    return ans