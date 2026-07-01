def solution(n, left, right):
    return [max(v // n, v % n) + 1 for v in range(left, right + 1)]