def solution(cards):
    n = len(cards)
    visited = [False] * (n + 1)
    g = []

    for idx in range(1, n + 1):
        if visited[idx]:
            continue

        cur = idx
        cnt = 0

        while not visited[cur]:
            visited[cur] = True
            cnt += 1
            cur = cards[cur - 1]

        g.append(cnt)

    g.sort(reverse=True)

    if len(g) < 2:
        return 0

    return g[0] * g[1]