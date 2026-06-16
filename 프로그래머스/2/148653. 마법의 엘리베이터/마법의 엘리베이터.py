def solution(storey):
    ans = 0
    
    while storey > 0:
        curr = storey % 10
        next_d = (storey // 10) % 10
        
        if curr > 5:
            ans += (10 - curr)
            storey = (storey // 10) + 1
        elif curr < 5:
            ans += curr
            storey //= 10
        else:
            ans += 5
            if next_d >= 5:
                storey = (storey // 10) + 1
            else:
                storey //= 10
                
    return ans