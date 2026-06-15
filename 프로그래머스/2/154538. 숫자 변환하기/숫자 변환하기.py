from collections import deque

def solution(x, y, n):
    if x == y:
        return 0

    queue = deque([(x, 0)])
    visited = set([x])
    
    while queue:
        curr, count = queue.popleft()
        
        for nxt in (curr + n, curr * 2, curr * 3):
            if nxt == y:
                return count + 1
            
            if nxt < y and nxt not in visited:
                visited.add(nxt)
                queue.append((nxt, count + 1))
                
    return -1