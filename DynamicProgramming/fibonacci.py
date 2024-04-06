def fibo_devide_c(n):
    # f(n) = f(n - 1) + f(n - 2)
    if n == 0 or n == 1:
        return n

    return fibo_devide_c(n - 1) + fibo_devide_c(n - 2)


def fibo_DP(n):
    dp = [0] * (n + 1)
    dp[1] = 1

    # 1. 구조적 특징 찾기
    # -> dp[n] = dp[n - 1] + dp[n - 2]

    # 작은문제부터 출발
    # 즉, 2부터 출발해서 마지막 원하는 fibo(10의 값을 만듬)
    for i in range(2, n + 1):
        dp[i] = dp[i - 1] + dp[i - 2]

    return dp


# 분할정복의 메모리제이션은 DP가 아니다
# 상향식이 아니기때문.
def fibo_dp_recur(n, memo={}):
    if n in memo:
        return memo[n]
    if n <= 1:
        return n
    memo[n] = fibo_dp_recur(n - 1, memo) + fibo_dp_recur(n - 2, memo)
    return memo[n]


result = fibo_DP(10)
result2 = fibo_dp_recur(10)
print(result)
print(result2)
