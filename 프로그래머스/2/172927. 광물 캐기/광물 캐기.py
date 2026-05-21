def solution(picks, minerals):
    dia, iron, stone = picks
    n = dia + iron + stone
    
    minerals = minerals[:n * 5]
    
    pocket = []
    for i in range(0, len(minerals), 5):
        chunk = minerals[i:i+5]
        cost = 0
        for m in chunk:
            if m == "diamond": cost += 25
            elif m == "iron": cost += 5
            else: cost += 1
        pocket.append((cost, chunk))
        
    pocket.sort(key=lambda x: x[0], reverse=True)
    
    ans = 0
    for cost, chunk in pocket:
        if dia > 0:
            dia -= 1
            ans += len(chunk)
        elif iron > 0:
            iron -= 1
            for m in chunk:
                ans += 5 if m == "diamond" else 1
        elif stone > 0:
            stone -= 1
            ans += cost
            
    return ans