def solution(temperature, t1, t2, a, b, onboard):
    INF = int(1e9)
    
    offset = 10
    temperature += offset
    t1 += offset
    t2 += offset
    
    dp = [INF] * 51
    dp[temperature] = 0
    
    for passenger in onboard[1:]:
        next_dp = [INF] * 51
        
        for curr_temp in range(51):
            if dp[curr_temp] == INF:
                continue
                
            for next_temp in range(51):
                if passenger == 1 and (next_temp < t1 or next_temp > t2):
                    continue
                
                cost = INF
                
                if next_temp == temperature:
                    if curr_temp == temperature:
                        cost = 0
                    elif curr_temp == temperature + 1 or curr_temp == temperature - 1:
                        cost = 0
                elif next_temp > temperature:
                    if curr_temp == next_temp - 1:
                        cost = a
                    elif curr_temp == next_temp:
                        cost = b
                    elif curr_temp == next_temp + 1:
                        cost = 0
                else:
                    if curr_temp == next_temp + 1:
                        cost = a
                    elif curr_temp == next_temp:
                        cost = b
                    elif curr_temp == next_temp - 1:
                        cost = 0
                
                if cost != INF:
                    next_dp[next_temp] = min(next_dp[next_temp], dp[curr_temp] + cost)
                    
        dp = next_dp
        
    return min(dp)