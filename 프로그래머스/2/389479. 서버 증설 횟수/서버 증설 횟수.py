from collections import deque

def solution(players, m, k):
    ans = 0
    server = deque()
    
    for player in players:
        for i in range(len(server)):
            server[i] -= 1
            
        while server and server[0] == 0:
            server.popleft()
            
        over = player - m
        if over >= 0:
            need = (over // m) + 1
            if need > len(server):
                req = need - len(server)
                ans += req
                server.extend([k] * req)
                

    return ans