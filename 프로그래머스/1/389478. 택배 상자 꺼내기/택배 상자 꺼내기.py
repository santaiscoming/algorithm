def solution(n, w, num):
    r, c = get_coords(num, w)
    m_r, _ = get_coords(n, w)
    
    ans = 0
    for r in range(r, m_r + 1):
        if r % 2 == 0:
            curr = r * w + c + 1
        else:
            curr = r * w + (w - 1 - c) + 1
            
        if curr <= n:
            ans += 1
            
    return ans

def get_coords(num, w):
        r = (num - 1) // w
        rest = (num - 1) % w
        
        if r % 2 == 0:
            col = rest
        else:
            col = (w - 1) - rest
        return r, col