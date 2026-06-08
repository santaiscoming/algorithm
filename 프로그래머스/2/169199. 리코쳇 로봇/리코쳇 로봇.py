from collections import deque

def solution(board):
    def move(r, c, d):
        DIREC = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        dr, dc = DIREC[d]
        
        
        while 0 <= r + dr < n and 0 <= c + dc < m and board[r + dr][c + dc] != 'D':
            r += dr
            c += dc
            
        return (r, c)
    
    n = len(board)
    m = len(board[0])
    
    sr, sc = 0, 0
    for r in range(n):
        for c in range(m):
            if board[r][c] == 'R':
                sr, sc = r, c

    visited = [[[False] * 4 for _ in range(m)] for _ in range(n)]
    q = deque()
    for d in range(4):
        q.append((sr, sc, d, 0))
        visited[r][c][d] = True
    
    while q:
        r, c, d, cnt = q.popleft()
        if board[r][c] == 'G':
            return cnt
        
        for d in range(4):
            nr, nc = move(r, c, d)
            if (nr != r or nc != c) and not visited[nr][nc][d]:
                visited[nr][nc][d] = True
                q.append((nr, nc, d, cnt + 1))
    
    return -1

