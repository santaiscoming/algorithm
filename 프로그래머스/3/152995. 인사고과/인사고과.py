def solution(scores):
    wanho_a, wanho_b = scores[0]
    wanho_sum = wanho_a + wanho_b
    
    scores.sort(key=lambda x: (-x[0], x[1]))
    
    max_b = 0
    rank = 1
    
    for a, b in scores:
        if b < max_b:
            if a == wanho_a and b == wanho_b:
                return -1
        else:
            max_b = max(max_b, b)
            if a + b > wanho_sum:
                rank += 1
                
    return rank