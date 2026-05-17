def solution(n, q, ans):
    answer = 0
    candidate = []

    def check():
        for query, expected in zip(q, ans):
            count = 0

            for num in candidate:
                if num in query:
                    count += 1

            if count != expected:
                return False

        return True

    def dfs(start):
        nonlocal answer

        if len(candidate) == 5:
            if check():
                answer += 1
            return

        need = 5 - len(candidate)

        for num in range(start, n + 1):
            if n - num + 1 < need:
                break

            candidate.append(num)
            dfs(num + 1)
            candidate.pop()

    dfs(1)

    return answer