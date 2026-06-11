from collections import deque

def solution(maps):
    n, m = len(maps), len(maps[0])
    
    start = lever = exit_pos = None
    for i in range(n):
        for j in range(m):
            if maps[i][j] == 'S':
                start = (i, j)
            elif maps[i][j] == 'L':
                lever = (i, j)
            elif maps[i][j] == 'E':
                exit_pos = (i, j)

    def bfs(start_pos, target_pos):
        q = deque([(start_pos[0], start_pos[1], 0)])
        vis = [[False] * m for _ in range(n)]
        vis[start_pos[0]][start_pos[1]] = True
        
        dr = [-1, 1, 0, 0]
        dc = [0, 0, -1, 1]
        
        while q:
            r, c, dist = q.popleft()
            
            if (r, c) == target_pos:
                return dist
                
            for i in range(4):
                nr, nc = r + dr[i], c + dc[i]

                if 0 <= nr < n and 0 <= nc < m and maps[nr][nc] != 'X' and not vis[nr][nc]:
                    vis[nr][nc] = True
                    q.append((nr, nc, dist + 1))
                    
        return -1

    to_lever = bfs(start, lever)
    if to_lever == -1:
        return -1
        
    to_exit = bfs(lever, exit_pos)
    if to_exit == -1:
        return -1
        
    return to_lever + to_exit