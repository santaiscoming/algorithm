from collections import deque
from math import isqrt


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

    max_id = max(max(row) for row in board)

    size = [0] * (max_id + 1)
    pos = [None] * (max_id + 1)
    count = [0] * (max_id + 1)
    min_r = [10**9] * (max_id + 1)
    min_c = [10**9] * (max_id + 1)

    for r in range(n):
        for c in range(m):
            app_id = board[r][c]

            if app_id == 0:
                continue

            count[app_id] += 1
            min_r[app_id] = min(min_r[app_id], r)
            min_c[app_id] = min(min_c[app_id], c)

    app_ids = []

    for app_id in range(1, max_id + 1):
        if count[app_id] == 0:
            continue

        app_ids.append(app_id)

        size[app_id] = isqrt(count[app_id])
        pos[app_id] = (min_r[app_id], min_c[app_id])

    def draw_board():
        new_board = [[0] * m for _ in range(n)]

        for app_id in app_ids:
            r, c = pos[app_id]
            s = size[app_id]

            for i in range(s):
                for j in range(s):
                    nr = (r + i) % n
                    nc = (c + j) % m
                    new_board[nr][nc] = app_id

        return new_board

    cur_board = draw_board()

    def get_front_cells(app_id, arrow):
        """
        app_id 앱을 arrow 방향으로 한 칸 밀 때
        새로 차지하게 되는 앞쪽 한 줄의 칸들을 반환한다.
        """
        r, c = pos[app_id]
        s = size[app_id]

        if arrow == 1:
            nc = (c + s) % m
            return [((r + i) % n, nc) for i in range(s)]

        if arrow == 2:
            nr = (r + s) % n
            return [(nr, (c + j) % m) for j in range(s)]

        if arrow == 3:
            nc = (c - 1) % m
            return [((r + i) % n, nc) for i in range(s)]

        nr = (r - 1) % n
        return [(nr, (c + j) % m) for j in range(s)]

    def get_push_group(start_id, arrow):
        group = {start_id}
        q = deque([start_id])

        while q:
            cur_id = q.popleft()

            for r, c in get_front_cells(cur_id, arrow):
                next_id = cur_board[r][c]

                if next_id == 0:
                    continue

                if next_id in group:
                    continue

                group.add(next_id)
                q.append(next_id)

        return group

    def move_group(group, arrow):
        nonlocal cur_board

        dr, dc = DIRS[arrow]

        for app_id in group:
            r, c = pos[app_id]
            pos[app_id] = ((r + dr) % n, (c + dc) % m)

        cur_board = draw_board()

    def find_broken_app(arrow):
        if arrow in (1, 3):
            for r in range(n):
                left_id = cur_board[r][0]
                right_id = cur_board[r][m - 1]

                if left_id == 0:
                    continue

                if left_id != right_id:
                    continue

                app_id = left_id
                s = size[app_id]
                _, c = pos[app_id]

                if s < m and c + s > m:
                    return app_id

        else:
            for c in range(m):
                top_id = cur_board[0][c]
                bottom_id = cur_board[n - 1][c]

                if top_id == 0:
                    continue

                if top_id != bottom_id:
                    continue

                app_id = top_id
                s = size[app_id]
                r, _ = pos[app_id]

                if s < n and r + s > n:
                    return app_id

        return None

    for start_id, arrow in commands:
        group = get_push_group(start_id, arrow)

        move_group(group, arrow)

        while True:
            broken_id = find_broken_app(arrow)

            if broken_id is None:
                break

            group = get_push_group(broken_id, arrow)
            move_group(group, arrow)

    return cur_board