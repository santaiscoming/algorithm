from collections import deque

def solution(n, wires):
    ans = n
    
    graph = [[] for _ in range(n + 1)]
    for v1, v2 in wires:
        graph[v1].append(v2)
        graph[v2].append(v1)
        
    for v1, v2 in wires:
        cnt1 = bfs(v1, v1, v2, graph, n)
        cnt2 = n - cnt1
        
        ans = min(ans, abs(cnt1 - cnt2))
        
    return ans

def bfs(start, v1, v2, graph, n):
    visited = [False] * (n + 1)
    q = deque([start])
    
    visited[start] = True
    count = 1
    
    while q:
        u = q.popleft()
        for v in graph[u]:
            if (u == v1 and v == v2) or (u == v2 and v == v1):
                continue
            if not visited[v]:
                visited[v] = True
                q.append(v)
                count += 1
                
    return count