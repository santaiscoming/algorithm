from collections import deque

def solution(maps):
    n = len(maps)
    m = len(maps[0])
    
    visited = [[False] * m for _ in range(n)]
    ans = []
    
    dr = [-1, 1, 0, 0]
    dc = [0, 0, -1, 1]
    
    for i in range(n):
        for j in range(m):
            if maps[i][j] != 'X' and not visited[i][j]:
                q = deque([(i, j)])
                visited[i][j] = True
                sum = int(maps[i][j])
                
                while q:
                    r, c = q.popleft()
                    
                    for k in range(4):
                        nr = r + dr[k]
                        nc = c + dc[k]
                        
                        if 0 <= nr < n and 0 <= nc < m:
                            if maps[nr][nc] != 'X' and not visited[nr][nc]:
                                visited[nr][nc] = True
                                sum += int(maps[nr][nc])
                                q.append((nr, nc))
                                
                ans.append(sum)
                
    if not ans:
        return [-1]
        
    return sorted(ans)