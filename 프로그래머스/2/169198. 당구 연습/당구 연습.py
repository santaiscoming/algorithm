def solution(m, n, startX, startY, balls):
    INF = float('inf')
    ans = []
    sr, sc = startY, startX
    
    for tc, tr in balls:
        dist = INF
        
        if not ((sc == tc) and (sr < tr)):
            top = (sr - (2 * n - tr)) ** 2 + (sc - tc) ** 2
            dist = min(dist, top)

        if not ((sc == tc) and (tr < sr)):
            bot = (sr - (-tr)) ** 2 + (sc - tc) ** 2
            dist = min(dist, bot)

        if not ((sr == tr) and (tc < sc)):
            left = (sr - tr) ** 2 + (sc - (-tc)) ** 2
            dist = min(dist, left)

        if not ((sr == tr) and (sc < tc)):
            right = (sr - tr) ** 2 + (sc - (2 * m - tc)) ** 2
            dist = min(dist, right)

        ans.append(dist)
        
        
    return ans