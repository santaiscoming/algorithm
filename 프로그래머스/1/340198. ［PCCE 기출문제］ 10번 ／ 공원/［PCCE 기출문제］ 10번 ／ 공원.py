def solution(mats, park):
    n = len(park)
    m = len(park[0])
    
    candi = []
    
    for mat in mats:
        for r in range(n):
            for c in range(m):
                if r + mat <= n and c + mat <= m:
                    is_place = True
                    for nr in range(r, r + mat):
                        for nc in range(c, c + mat):
                            if park[nr][nc] != "-1":
                                is_place = False
                                
                    if is_place:
                        candi.append(mat)
                        
                        
    return -1 if not candi else max(candi)