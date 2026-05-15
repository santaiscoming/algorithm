from collections import deque

def solution(h, grid, panels, seqs):
    INF = 10 ** 15

    n = len(grid)
    m = len(grid[0])
    k = len(panels)

    DIRS = [(-1, 0), (0, 1), (1, 0), (0, -1)]

    er = -1
    ec = -1
    for r in range(n):
        for c in range(m):
            if grid[r][c] == '@':
                er, ec = r, c
    
    pos = [(er, ec)]
    for f, r, c in panels:
        pos.append((r - 1, c - 1))

    floors = [0]  
    for f, r, c in panels:
        floors.append(f)

    def bfs(start_r, start_c):
        dist = [[-1] * m for _ in range(n)]
        q = deque()

        dist[start_r][start_c] = 0
        q.append((start_r, start_c))

        while q:
            r, c = q.popleft()

            for dr, dc in DIRS:
                nr = r + dr
                nc = c + dc

                if nr < 0 or nr >= n or nc < 0 or nc >= m:
                    continue

                if grid[nr][nc] == '#':
                    continue

                if dist[nr][nc] != -1:
                    continue

                dist[nr][nc] = dist[r][c] + 1
                q.append((nr, nc))

        return dist

    same_dist = [[0] * (k + 1) for _ in range(k + 1)]

    for i in range(k + 1):
        sr, sc = pos[i]
        d = bfs(sr, sc)

        for j in range(k + 1):
            tr, tc = pos[j]
            same_dist[i][j] = d[tr][tc]

    def get_cost(a, b):
        if floors[a] == floors[b]:
            return same_dist[a][b]

        return (
            same_dist[a][0]
            + abs(floors[a] - floors[b])
            + same_dist[0][b]
        )

    pre = [0] * (k + 1)
    for a, b in seqs:
        pre[b] |= 1 << (a - 1)

    dp = [[INF] * (k + 1) for _ in range(1 << k)]
    dp[0][1] = 0

    for mask in range(1 << k):
        for pos in range(1, k + 1):
            if dp[mask][pos] == INF:
                continue

            for nxt in range(1, k + 1):
                bit = 1 << (nxt - 1)
                if mask & bit:
                    continue
                if (mask & pre[nxt]) != pre[nxt]:
                    continue

                next_mask = mask | bit
                cost = get_cost(pos, nxt)

                dp[next_mask][nxt] = min(
                    dp[next_mask][nxt],
                    dp[mask][pos] + cost
                )

    full = (1 << k) - 1
    return min(dp[full][pos] for pos in range(1, k + 1))