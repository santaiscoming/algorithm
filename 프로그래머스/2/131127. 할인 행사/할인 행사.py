from collections import defaultdict

def solution(want, number, discount):
    buy = defaultdict(int)
    for i in range(len(want)):
        name = want[i]
        cnt = number[i]
        buy[name] = cnt
        
    stock = defaultdict(int)
    for i in range(10):
        name = discount[i]
        stock[name] += 1
    
    ans = 0
    
    for l in range(len(discount)):
        r = l + 10
        
        if all(stock[name] >= buy[name] and stock[name] > 0 for name in buy.keys()):
            ans += 1
        
        stock[discount[l]] -= 1
        if r < len(discount):
            stock[discount[r]] += 1
        
    return ans