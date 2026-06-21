def solution(storey):
    ans = 0

    while storey:
        cur = storey % 10
        nxt = (storey // 10) % 10

        if cur > 5 or (cur == 5 and nxt >= 5):
            ans += 10 - cur
            storey += 10
        else:
            ans += cur

        storey //= 10

    return ans