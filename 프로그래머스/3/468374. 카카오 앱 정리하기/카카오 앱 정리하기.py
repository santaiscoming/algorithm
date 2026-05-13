from collections import deque

def solution(board, commands):
    n = len(board)
    m = len(board[0])
    DIRS = [(0, 1), (1, 0), (0, -1), (-1, 0)]
    
    def get_dup_ids(id, arrow):
        q = deque([id])
        ret = set([id])
        
        while q:
            curr = q.popleft()
            
            for r in range(n):
                for c in range(m):
                    dr, dc = DIRS[arrow - 1]
                    nr = (r + dr) % n
                    nc = (c + dc) % m
                    
                    next = board[nr][nc]
                    if (board[r][c] == curr and
                        not next in ret and
                        curr != next and
                        next != 0):
                        q.append(next)
                        ret.add(next)
                        
        return ret
    
    def move_apps(ids, arrow):
        dr, dc = DIRS[arrow - 1]
        cells = []
        for r in range(n):
            for c in range(m):
                if board[r][c] in ids:
                    cells.append((r, c, board[r][c]))

        for r, c, id in cells:
            board[r][c] = 0

        for r, c, id in cells:
            nr = (r + dr) % n
            nc = (c + dc) % m
            board[nr][nc] = id
            
    def get_broken_app(arrow):
        if arrow in [1, 3]:
            for r in range(n):
                l_id = board[r][0]
                r_id = board[r][m - 1]

                if l_id == 0 or l_id != r_id:
                    continue

                is_broken = False
                for c in range(m):
                    if board[r][c] != l_id:
                        return l_id
        else:
            for c in range(m):
                t = board[0][c]
                b = board[n - 1][c]

                if t == 0 or t != b:
                    continue

                for r in range(n):
                    if board[r][c] != t:
                        return t

        return None
    
    for id, arrow in commands:
        ids = get_dup_ids(id, arrow)
        move_apps(ids, arrow)

        while True:
            broken_app = get_broken_app(arrow)
            if broken_app is None:
                break

            ids = get_dup_ids(broken_app, arrow)
            move_apps(ids, arrow)

    return board