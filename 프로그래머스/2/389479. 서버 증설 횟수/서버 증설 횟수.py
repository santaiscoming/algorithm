from collections import deque
def solution(players, m, k):
    ans = 0
    server = deque()
    
    for player in players:
        curr_cap = (len(server) + 1) * m
        
        if player >= curr_cap:
            inc = ((player - curr_cap) // m) + 1
            ans += inc
            for _ in range(inc):
                server.append(k)
            
        remove = 0
        for i, s in enumerate(server):
            server[i] -= 1
            if server[i] == 0:
                remove += 1
        
        for _ in range(remove):
            server.popleft()
                    
    return ans