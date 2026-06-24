def solution(order):
    s = []
    i = 0
    
    for box in range(1, len(order) + 1):
        s.append(box)
        
        while s and s[-1] == order[i]:
            s.pop()
            i += 1
            
    return i