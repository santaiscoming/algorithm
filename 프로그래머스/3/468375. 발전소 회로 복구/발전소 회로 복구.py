from collections import deque

def solution(h, grid, panels, seqs):
    INF = float('inf')
    DIRS = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    n = len(grid)
    m = len(grid[0])
    k = len(panels)
    
    def bfs(r, c):
        dist = [[-1] * m for _ in range(n)]
        dist[r][c] = 0
        
        q = deque([(r, c)])
        while q:
            cr, cc = q.popleft()
            
            for dr, dc in DIRS: 
                nr = cr + dr
                nc = cc + dc
                
                if (0 <= nr < n and
                    0 <= nc < m and
                    dist[nr][nc] == -1 and
                    grid[nr][nc] != '#'):
                    dist[nr][nc] = dist[cr][cc] + 1
                    q.append((nr, nc))
            
        return dist
    
    er = ec = None
    for r in range(n):
        for c in range(m):
            if grid[r][c] == '@':
                er = r
                ec = c
                
    pos = [(er, ec)]
    for _, r, c in panels:
        pos.append((r - 1, c - 1))
    
    dist = [[-1] * (k + 1) for _ in range(k + 1)]
    for i in range(k + 1):
        sr, sc = pos[i]
        from_i = bfs(sr, sc)
        for j in range(k + 1):
            tr, tc = pos[j]
            dist[i][j] = from_i[tr][tc]
    
    floors = [0] * (k + 1)
    for i, (f, _, _) in enumerate(panels):
        floors[i + 1] = f
        
    def get_cost(i, j):
        if floors[i] == floors[j]:
            return dist[i][j]
        
        diff = abs(floors[i] - floors[j])
        return dist[i][0] + diff + dist[0][j]
        
    
    order = [0] * (k + 1)
    for a, b in seqs:
        order[b] |= 1 << (a - 1)

    dp = [[INF] * (k + 1) for _ in range(1 << k)]
    mask = 0
    start = 1
    dp[mask][start] = 0
    
    # 가능한 모든 경우를 탐색해야하므로
    for mask in range(1 << k):
        for i in range(1, k + 1):
            if dp[mask][i] == INF:
                continue

            for nxt in range(1, k + 1):
                bit = 1 << (nxt - 1)

                if mask & bit:
                    continue
                if (mask & order[nxt]) != order[nxt]:
                    continue

                next_mask = mask | bit
                cost = get_cost(i, nxt)

                dp[next_mask][nxt] = min(
                    dp[next_mask][nxt],
                    dp[mask][i] + cost
                )
    
    full = (1 << k) - 1
    answer = min(dp[full][pos] for pos in range(1, k + 1))

    return answer    