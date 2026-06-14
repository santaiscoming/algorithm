from collections import deque

def solution(maps):
    DIREC = [(-1, 0), (1, 0), (0, 1), (0, -1)]
    n = len(maps)
    m = len(maps[0])
    answer = 0
    
    def bfs(sr, sc, er, ec):
        visited = [[False] * m for _ in range(n)]
        q = deque()
        
        q.append((sr, sc, 0))
        visited[sr][sc] = True
        
        while q:
            (r, c, d) = q.popleft()
            
            if maps[r][c] == maps[er][ec]:
                return d
            
            for dr, dc in DIREC:
                nr = dr + r
                nc = dc + c
                
                if 0 <= nr < n and 0 <= nc < m and not visited[nr][nc] and maps[nr][nc] != 'X':
                    q.append((nr, nc, d + 1))
                    visited[nr][nc] = True
                    
        return -1
    
    s = (0, 0)
    via = (0, 0)
    dest = (0, 0)
    for r in range(n):
        for c in range(m):
            if maps[r][c] == "L":
                via = (r, c)
            if maps[r][c] == "S":
                s = (r, c)
            if maps[r][c] == "E":
                dest = (r, c)
    
    print(s, via, dest)
    to_via = bfs(s[0], s[1], via[0], via[1])
    to_dest = bfs(via[0], via[1], dest[0], dest[1])
    
    return to_via + to_dest if all(v > 0 for v in [to_via, to_dest]) else -1