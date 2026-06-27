def solution(n, k, cmd):
    prev = [i - 1 for i in range(n)]
    next = [i + 1 for i in range(n)]
    next[n - 1] = -1
    
    s = []
    ptr = k
    
    for c in cmd:
        if c[0] == 'U':
            _, step = c.split()
            step = int(step)
            for _ in range(step):
                ptr = prev[ptr]
        elif c[0] == 'D':
            _, step = c.split() 
            step = int(step)
            for _ in range(step):
                ptr = next[ptr]
        elif c[0] == 'C':
            s.append((ptr, prev[ptr], next[ptr]))
            
            p, nxt = prev[ptr], next[ptr]
            
            if p != -1:
                next[p] = nxt
            if nxt != -1:
                prev[nxt] = p
                
            ptr = nxt if nxt != -1 else p
            
        elif c[0] == 'Z':
            r, p, nxt = s.pop()
            
            if p != -1:
                next[p] = r
            if nxt != -1:
                prev[nxt] = r

    result = ["O"] * n
    for r, _, _ in s:
        result[r] = "X"
        
    return "".join(result)