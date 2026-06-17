def solution(n, l, r):
    return get(n, r) - get(n, l - 1)

def get(n, k):
    if n == 0: 
        return 1 if k > 0 else 0

    l = 5 ** (n - 1)
    one = 4 ** (n - 1)

    part = k // l
    remain = k % l

    if part < 2:
        ret = part * one
    elif part == 2:
        ret = 2 * one
    else:
        ret = (part - 1) * one

    if part == 2:
        return ret
    else:
        return ret + get(n - 1, remain)
