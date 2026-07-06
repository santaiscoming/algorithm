def solution(numbers):
    ans = []

    for x in numbers:
        bit = 1

        while x & bit:
            bit <<= 1

        ans.append((x | bit) & ~(bit >> 1))

    return ans