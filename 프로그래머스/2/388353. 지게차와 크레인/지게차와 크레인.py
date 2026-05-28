from collections import deque

def solution(storage, requests):
    dr = [-1, 1, 0, 0]
    dc = [0, 0, -1, 1]
    
    n = len(storage)
    m = len(storage[0])
    
    grid = [[' '] * (m + 2) for _ in range(n + 2)]
    for r in range(n):
        for c in range(m):
            grid[r + 1][c + 1] = storage[r][c]
            

    
    for req in requests:
        target = req[0]
        
        if len(req) == 2:
            for r in range(1, n + 1):
                for c in range(1, m + 1):
                    if grid[r][c] == target:
                        grid[r][c] = ' '
        else:
            q = deque([(0, 0)])
            visited = [[False] * (m + 2) for _ in range(n + 2)]
            visited[0][0] = True
            
            remove = []
            
            while q:
                r, c = q.popleft()
                
                for i in range(4):
                    nr, nc = r + dr[i], c + dc[i]
                    
                    if 0 <= nr < n + 2 and 0 <= nc < m + 2 and not visited[nr][nc]:
                        if grid[nr][nc] == ' ':
                            visited[nr][nc] = True
                            q.append((nr, nc))
                        elif grid[nr][nc] == target:
                            visited[nr][nc] = True
                            remove.append((nr, nc))
                            
            for r, c in remove:
                grid[r][c] = ' '
                
    cnt = 0
    for r in range(1, n + 1):
        for c in range(1, m + 1):
            if grid[r][c] != ' ':
                cnt += 1
                
    return cnt