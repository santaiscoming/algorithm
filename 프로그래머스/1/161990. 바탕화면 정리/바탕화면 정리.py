def solution(wallpaper):
    INF = float('inf')
    n = len(wallpaper)
    m = len(wallpaper[0])
    
    sr, sc, er, ec = INF, INF, 0, 0
    for r in range(n):
        for c in range(m):
            if wallpaper[r][c] == '#':
                if sr > r:
                    sr = r
                if sc > c:
                    sc = c
                if er < r:
                    er = r
                if ec < c:
                    ec = c
            
    return [sr, sc, er + 1, ec + 1]