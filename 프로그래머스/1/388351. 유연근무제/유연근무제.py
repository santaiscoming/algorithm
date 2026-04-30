def solution(schedules, timelogs, startday):
    ans = 0
    
    for i in range(len(schedules)):
        deadline = parse(schedules[i]) + 10
        is_suc = True
        
        for j in range(7):
            day = (startday - 1 + j) % 7 + 1
            
            if day in (6, 7):
                continue
                
            arrival = parse(timelogs[i][j])
            
            if arrival > deadline:
                is_suc = False
                break
                
        if is_suc:
            ans += 1
            
    return ans
            
def parse(t):
    h = t // 100
    m = t % 100
    return h * 60 + m