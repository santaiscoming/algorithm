from collections import deque

def solution(maps):
    DIRS = [(1, 0), (-1, 0), (0, 1), (0, -1)]
    n = len(maps)
    m = len(maps[0])
    
    answer = []
    
    def bfs(sr, sc, visited):
        q = deque()
        
        ret = int(maps[sr][sc])
        visited[sr][sc] = True
        q.append((sr, sc))
        
        while q:
            r, c = q.popleft()
            
            for dr, dc in DIRS:
                nr = dr + r
                nc = dc + c
                
                if ((0 <= nr < n) and
                    (0 <= nc < m) and
                    (not visited[nr][nc]) and
                    (maps[nr][nc] != 'X')):
                        q.append((nr, nc))
                        visited[nr][nc] = True
                        ret += int(maps[nr][nc])
                    
        return ret
    
    ans = []
    visited = [[False] * m for _ in range(n)]
    for r in range(n):
        for c in range(m):
            if not visited[r][c] and maps[r][c] != 'X':
                res = bfs(r, c, visited)
                ans.append(res)
        
    
    return [-1] if not ans else sorted(ans)