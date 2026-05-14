def solution(wallet, bill):
    cnt = 0
    
    w_min, w_max = min(wallet), max(wallet)
    b_min, b_max = min(bill), max(bill)
    
    while b_min > w_min or b_max > w_max:
        b_max //= 2
        
        b_min, b_max = min(b_min, b_max), max(b_min, b_max)
        
        cnt += 1
        
    return cnt