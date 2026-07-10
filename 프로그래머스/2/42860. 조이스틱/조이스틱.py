def solution(name):
    up_down = 0
    for char in name:
        up_down += min(ord(char) - ord('A'), ord('Z') - ord(char) + 1)
    
    n = len(name)
    move = n - 1
    
    for i in range(n):
        nxt = i + 1
        while nxt < n and name[nxt] == 'A':
            nxt += 1
            
        move = min([
            move,
            i + i + (n - nxt),
            (n - nxt) * 2 + i
        ])
        
    return up_down + move