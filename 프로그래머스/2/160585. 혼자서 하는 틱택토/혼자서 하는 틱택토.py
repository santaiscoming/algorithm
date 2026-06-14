def solution(board):
    o_cnt = 0
    x_cnt = 0
    for r in range(3):
        for c in range(3):
            if board[r][c] == 'O':
                o_cnt += 1
            if board[r][c] == 'X':
                x_cnt += 1
    
    if x_cnt > o_cnt:
        return 0
    if o_cnt - 1 > x_cnt:
        return 0
    
    for r in range(3):
        if all([board[r][c] == 'O' for c in range(3)]) and x_cnt >= o_cnt:
            return 0
        if all([board[r][c] == 'X' for c in range(3)]) and x_cnt < o_cnt:
            return 0
        if all([board[c][r] == 'O' for c in range(3)]) and x_cnt >= o_cnt:
            return 0
        if all([board[c][r] == 'X' for c in range(3)]) and x_cnt < o_cnt:
            return 0
        
    if all([board[c][c] == 'O' for c in range(3)]) and x_cnt >= o_cnt:
        return 0
    if all([board[c][c] == 'X' for c in range(3)]) and x_cnt < o_cnt:
        return 0
    if all([board[2 - c][c] == 'O' for c in range(3)]) and x_cnt >= o_cnt:
        return 0
    if all([board[2 - c][c] == 'X' for c in range(3)]) and x_cnt < o_cnt:
        return 0
    
    return 1