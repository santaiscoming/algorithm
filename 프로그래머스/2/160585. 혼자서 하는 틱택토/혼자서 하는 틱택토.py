def solution(board):
    board = [list(row) for row in board]

    o_cnt = sum(row.count('O') for row in board)
    x_cnt = sum(row.count('X') for row in board)

    if not (o_cnt == x_cnt or o_cnt == x_cnt + 1):
        return 0

    o_win = win(board, 'O')
    x_win = win(board, 'X')

    if o_win and x_win:
        return 0

    if o_win and o_cnt != x_cnt + 1:
        return 0

    if x_win and o_cnt != x_cnt:
        return 0

    return 1

def win(board, t):
    for row in board:
        if row == [t, t, t]:
            return True
        
    for col in range(3):
        if [board[row][col] for row in range(3)] == [t, t, t]:
            return True
        
    if [board[0][0], board[1][1], board[2][2]] == [t, t, t]:
        return True
    
    if [board[2][0], board[1][1], board[0][2]] == [t, t, t]:
        return True
    
    return False