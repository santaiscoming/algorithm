from collections import Counter

def solution(weights):
    answer = 0
    cnt = Counter(weights)
    
    for w in cnt:
        if cnt[w] > 1:
            n = cnt[w]
            answer += (n * (n - 1)) // 2
        
        if w * 3 / 2 in cnt:
            answer += cnt[w] * cnt[w * 3 / 2]
            
        if w * 2 in cnt:
            answer += cnt[w] * cnt[w * 2]
            
        if w * 4 / 3 in cnt:
            answer += cnt[w] * cnt[w * 4 / 3]
            
    return answer