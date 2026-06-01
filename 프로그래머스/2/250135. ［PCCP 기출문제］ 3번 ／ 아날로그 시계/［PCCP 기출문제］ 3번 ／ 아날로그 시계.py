def solution(h1, m1, s1, h2, m2, s2):
    ans = 0
    start = h1 * 3600 + m1 * 60 + s1
    end = h2 * 3600 + m2 * 60 + s2
    
    h_start = (start / 120) % 360
    m_start = (start / 10) % 360
    s_start = (start * 6) % 360
    
    if s_start == h_start or s_start == m_start:
        ans += 1

    for t in range(start, end):
        h_curr = (t / 120) % 360
        m_curr = (t / 10) % 360
        s_curr = (t * 6) % 360
        
        h_next = ((t + 1) / 120) % 360
        m_next = ((t + 1) / 10) % 360
        s_next = ((t + 1) * 6) % 360
        
        if h_next == 0: h_next = 360
        if m_next == 0: m_next = 360
        if s_next == 0: s_next = 360
            
        if s_curr < h_curr and s_next >= h_next:
            ans += 1
            
        if s_curr < m_curr and s_next >= m_next:
            ans += 1
            
        if s_next == h_next and s_next == m_next:
            ans -= 1
            
    return ans