def solution(n, left, right):
    return [max(k % n, k // n) + 1 for k in range(left, right + 1)]