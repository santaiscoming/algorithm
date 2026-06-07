def solution(board, h, w):
    DIREC = [(1, 0), (0, 1), (-1, 0), (0, -1)]
    n = len(board)
    m = len(board[0])
    
    ans = 0
    target = board[h][w]
    
    cr, cc = h, w
    for dr, dc in DIREC:
        nr = dr + cr
        nc = dc + cc
        
        if 0 <= nr < n and 0 <= nc < m and target == board[nr][nc]:
            ans += 1
            
    return ans