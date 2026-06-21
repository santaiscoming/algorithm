def solution(n, l, r):
    def count(idx, depth):
        if idx <= 0:
            return 0

        if depth == 0:
            return 1

        total = 5 ** (depth - 1)
        ones = 4 ** (depth - 1)

        calced = idx // total
        rest = idx % total

        if calced <= 2:
            cnt = calced * ones
        else:
            cnt = (calced - 1) * ones

        if calced == 2:
            return cnt

        return cnt + count(rest, depth - 1)

    return count(r, n) - count(l - 1, n)