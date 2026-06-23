from collections import Counter

def solution(k, tangerine):
    answer = 0
    counter = sorted(list(Counter(tangerine).items()), key=lambda x: -x[1])
    
    for t, cnt in counter:
        if k > cnt:
            k -= cnt
            answer += 1
        else:
            answer += 1
            break
    
    return answer