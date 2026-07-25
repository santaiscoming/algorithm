def solution(n, k):
    k_num = ""
    while n > 0:
        k_num = str(n % k) + k_num
        n //= k
        
    candidates = k_num.split('0')
    
    answer = 0
    for cand in candidates:
        if cand:
            if is_prime(int(cand)):
                answer += 1
                
    return answer

def is_prime(num):
    if num < 2:
        return False
    
    for i in range(2, int(num**0.5) + 1):
        if num % i == 0:
            return False
        
    return True
