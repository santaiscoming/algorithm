def solution(w, h):
    return (w * h) - (w + h - gcd(w, h))

def gcd(a, b):
    if b == 0:
        return a
    return gcd(b, a % b)
        