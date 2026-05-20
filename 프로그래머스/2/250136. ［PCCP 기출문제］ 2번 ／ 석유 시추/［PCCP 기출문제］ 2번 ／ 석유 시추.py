from collections import deque

def solution(land):
    DIRS = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    n = len(land)
    m = len(land[0])

    visited = [[False] * m for _ in range(n)]
    oil = [0] * m

    def bfs(sr, sc):
        q = deque()
        q.append((sr, sc))
        visited[sr][sc] = True

        size = 0
        cols = set()

        while q:
            r, c = q.popleft()

            size += 1
            cols.add(c)

            for dr, dc in DIRS:
                nr = r + dr
                nc = c + dc

                if nr < 0 or nr >= n or nc < 0 or nc >= m:
                    continue
                if visited[nr][nc]:
                    continue
                if land[nr][nc] == 0:
                    continue

                visited[nr][nc] = True
                q.append((nr, nc))

        return size, cols

    for r in range(n):
        for c in range(m):
            if land[r][c] == 1 and not visited[r][c]:
                size, cols = bfs(r, c)

                for col in cols:
                    oil[col] += size

    return max(oil)