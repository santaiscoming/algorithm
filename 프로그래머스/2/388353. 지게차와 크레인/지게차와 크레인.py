from collections import deque

def solution(storage, requests):
    n = len(storage)
    m = len(storage[0])
    DIRS = [[1, 0], [0, 1], [-1, 0], [0, -1]]
    
    mat = [['.'] * (m + 2) for _ in range(n + 2)]
    for r in range(n):
        for c in range(m):
            mat[r + 1][c + 1] = storage[r][c]

    cnt = 0
    for target in requests:
        if len(target) == 2:
            t = target[0]
            for r in range(n + 2):
                for c in range(m + 2):
                    if mat[r][c] == t:
                        mat[r][c] = '.'
                        cnt += 1
        else:
            s = (0, 0)
            q = deque()
            q.append(s)
            to_remove = []
            visited = [[False] * (m + 2) for _ in range(n + 2)] 
            visited[0][0] = False
            while q:
                cr, cc = q.popleft()

                for dr, dc in DIRS:
                    nr = cr + dr
                    nc = cc + dc
                    if (0 <= nr < n + 2 and
                        0 <= nc < m + 2 and
                        not visited[nr][nc]):
                            if mat[nr][nc] == '.':
                                q.append((nr, nc))
                                visited[nr][nc] = True
                            if mat[nr][nc] == target:
                                to_remove.append((nr, nc))  
                                visited[nr][nc] = True

            for r, c in to_remove:
                mat[r][c] = '.'
                cnt += 1
        
    return n * m - cnt