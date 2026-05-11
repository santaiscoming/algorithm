from collections import deque

def solution(board, commands):
    n = len(board)
    m = len(board[0])
    DIRS = [
        None,
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ]

    board = [row[:] for row in board]

    def get_group(s_id, arrow):
        dr, dc = DIRS[arrow]

        group = {s_id}
        q = deque([s_id])

        while q:
            curr = q.popleft()

            for r in range(n):
                for c in range(m):
                    if board[r][c] != curr:
                        continue

                    nr = (r + dr) % n
                    nc = (c + dc) % m

                    next = board[nr][nc]

                    if next != 0 and next not in group:
                        group.add(next)
                        q.append(next)

        return group

    def move_group(group, arrow):
        dr, dc = DIRS[arrow]
        cells = []

        for r in range(n):
            for c in range(m):
                if board[r][c] in group:
                    cells.append((r, c, board[r][c]))

        for r, c, _id in cells:
            board[r][c] = 0

        for r, c, _id in cells:
            nr = (r + dr) % n
            nc = (c + dc) % m
            board[nr][nc] = _id

    def find_broken_apps(arrow):
        broken = []
        added = set()

        if arrow in (1, 3):
            for r in range(n):
                left_id = board[r][0]
                right_id = board[r][m - 1]

                if left_id == 0:
                    continue

                if left_id != right_id:
                    continue

                is_broken = False
                for c in range(m):
                    if board[r][c] != left_id:
                        is_broken = True
                        break

                if is_broken and left_id not in added:
                    added.add(left_id)
                    broken.append(left_id)

        else:
            for c in range(m):
                top_id = board[0][c]
                bottom_id = board[n - 1][c]

                if top_id == 0:
                    continue

                if top_id != bottom_id:
                    continue

                is_broken = False
                for r in range(n):
                    if board[r][c] != top_id:
                        is_broken = True
                        break

                if is_broken and top_id not in added:
                    added.add(top_id)
                    broken.append(top_id)

        return broken


    for id, arrow in commands:
        group = get_group(id, arrow)
        move_group(group, arrow)

        while True:
            broken_apps = find_broken_apps(arrow)

            if not broken_apps:
                break

            broken_id = broken_apps[0]
            group = get_group(broken_id, arrow)
            move_group(group, arrow)

    return board