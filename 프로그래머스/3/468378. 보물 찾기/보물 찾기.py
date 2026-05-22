import sys

def solution(depth, money, excavate):
    n = len(depth)

    INF = float('inf')
    dp = [[0] * n for _ in range(n)]
    picked = [[-1] * n for _ in range(n)]

    for length in range(1, n + 1):
        for l in range(n - length + 1):
            r = l + length - 1

            best = INF
            best_k = -1

            for k in range(l, r + 1):
                left = dp[l][k - 1] if k > l else 0
                right = dp[k + 1][r] if k < r else 0

                cost = depth[k] + max(left, right)

                if cost < best:
                    best = cost
                    best_k = k

            dp[l][r] = best
            picked[l][r] = best_k

    l, r = 0, n - 1

    while l <= r:
        k = picked[l][r]

        res = excavate(k + 1)

        if res == 0:
            return k + 1

        elif res == -1:
            r = k - 1

        else:
            l = k + 1

    return -1