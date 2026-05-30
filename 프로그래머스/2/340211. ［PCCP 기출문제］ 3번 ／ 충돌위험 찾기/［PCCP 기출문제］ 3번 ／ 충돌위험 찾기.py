from collections import Counter

def solution(points, routes):
    ptr = {i + 1: (r, c) for i, (r, c) in enumerate(points)}
    visit = Counter()
    
    for route in routes:
        t = 0
        r, c = ptr[route[0]]
        visit[(t, r, c)] += 1
        
        for i in range(1, len(route)):
            nr, nc = ptr[route[i]]
            
            while r != nr:
                r += 1 if nr > r else -1
                t += 1
                visit[(t, r, c)] += 1
                
            while c != nc:
                c += 1 if nc > c else -1
                t += 1
                visit[(t, r, c)] += 1
                
    return sum(1 for cnt in visit.values() if cnt >= 2)