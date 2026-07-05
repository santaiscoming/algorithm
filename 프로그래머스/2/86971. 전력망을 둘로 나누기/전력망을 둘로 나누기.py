from collections import deque, defaultdict

def solution(n, wires):
    m = len(wires)
    ans = float('inf')
    
    for cut in range(m):
        cand = []
        graph = make(cut, wires)
        visited = [False] * (n + 1)
        
        for s in range(1, n + 1):
            if not visited[s]:
                cand.append(bfs(s, visited, graph))
        
        if len(cand) > 1:
            ans = min(ans, abs(cand[0] - cand[1]))
        
        # print(cut, cand)
        
    return ans

def make(cut, edges):
    ret = defaultdict(list)
    
    for i, edge in enumerate(edges):
        if i == cut:
            continue
        
        u, v = edges[i]
        ret[u].append(v)
        ret[v].append(u)
    
    return ret
        

def bfs(s, visited, graph):
    q = deque()
    
    visited[s] = True
    q.append(s)
    ret = 0
    
    while q:
        u = q.popleft()
        
        for v in graph[u]:
            if not visited[v]:
                q.append(v)
                visited[v] = True
                ret += 1
                
    return ret
                