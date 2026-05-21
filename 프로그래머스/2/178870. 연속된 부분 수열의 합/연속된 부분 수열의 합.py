def solution(sequence, k):
    INF = float('inf')
    n = len(sequence)

    l = 0
    acc = 0
    best = float('inf')
    answer = [0, 0]

    for r in range(n):
        acc += sequence[r]

        while acc >= k:
            if acc == k:
                leng = r - l
                
                if leng < best:
                    best = leng
                    answer = [l, r]

            acc -= sequence[l]
            l += 1

    return answer