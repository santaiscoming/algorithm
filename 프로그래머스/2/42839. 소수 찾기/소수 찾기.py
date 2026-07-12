from itertools import permutations

def solution(numbers):
    nums = set()
    
    for i in range(1, len(numbers) + 1):
        for p in permutations(numbers, i):
            num = int(''.join(p))
            nums.add(num)
            
    return  sum(1 for num in nums if is_prime(num))

def is_prime(n):
    if n < 2:
        return False
    for i in range(2, int(n**0.5) + 1):
        if n % i == 0:
            return False
    return True
