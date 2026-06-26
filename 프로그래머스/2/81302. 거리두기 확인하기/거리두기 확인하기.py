from collections import deque

def solution(places):
    ans = []
    for p in places:

        is_good = True
        for r in range(5):
            for c in range(5):
                if p[r][c] == 'P':
                    if not bfs(r, c, p):
                        is_good = False
                    
        if is_good:
            ans.append(1)
        else:
            ans.append(0)
            
    
    return ans

def bfs(sr, sc, board):
    DIRS = [(1, 0), (0, 1), (-1 ,0), (0, -1)]
    
    visited = [[False] * 5 for _ in range(5)]
    q = deque()
    
    visited[sr][sc] = True
    q.append((sr, sc, 0))
    
    while q:
        r, c, d = q.popleft()
        
        if board[r][c] == 'P' and d >= 1:
            return False
        
        for dr, dc in DIRS:
            nr = dr + r
            nc = dc + c
            
            if (0 <= nr < 5 and
                0 <= nc < 5 and
                not visited[nr][nc] and
                board[nr][nc] != 'X' and
                d < 2):
                visited[nr][nc] = True
                q.append((nr, nc, d + 1))
                
    return True