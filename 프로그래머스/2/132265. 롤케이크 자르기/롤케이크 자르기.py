from collections import Counter

def solution(topping):
    answer = 0
    
    right = Counter(topping)
    left = set()
    
    for t in topping:
        left.add(t)
        
        right[t] -= 1
        if right[t] == 0:
            del right[t]
            
        if len(left) == len(right):
            answer += 1
            
    return answer