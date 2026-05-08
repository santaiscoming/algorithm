def solution(dist_limit, split_limit):
    answer = 1

    def dfs(cur, used, split, leaf):
        nonlocal answer

        if used > dist_limit:
            return

        answer = max(answer, leaf + cur)

        for child in range(2, 4):
            next_split = split * child

            if next_split > split_limit:
                continue

            next_nodes = cur * child
            remain = dist_limit - used

            next_cur = min(next_nodes, remain)
            next_leaf = leaf + (next_nodes - next_cur)

            dfs(
                next_cur,
                used + next_cur,
                next_split,
                next_leaf
            )

    dfs(1, 1, 1, 0)

    return answer