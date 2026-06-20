import heapq

def solution(n, k, enemy):
    h = []
    
    for i, e in enumerate(enemy):
        heapq.heappush(h, -e)
        n -= e
        
        if n < 0:
            if k > 0:
                most_enemy = -heapq.heappop(h)
                n += most_enemy
                k -= 1
            else:
                return i
                
    return len(enemy)