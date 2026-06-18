def solution(x, y, n):
    INF = float('inf')
    dp = [INF] * (y + 1)
    dp[x] = 0

    for cur in range(x, y + 1):
        if dp[cur] == INF:
            continue

        if cur + n <= y:
            dp[cur + n] = min(dp[cur + n], dp[cur] + 1)

        if cur * 2 <= y:
            dp[cur * 2] = min(dp[cur * 2], dp[cur] + 1)

        if cur * 3 <= y:
            dp[cur * 3] = min(dp[cur * 3], dp[cur] + 1)

    return -1 if dp[y] == INF else dp[y]