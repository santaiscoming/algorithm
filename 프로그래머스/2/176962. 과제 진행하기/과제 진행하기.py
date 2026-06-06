def solution(plans):
    parsed_plans = []
    for name, start, play_time in plans:
        h, m = map(int, start.split(':'))
        start_min = h * 60 + m
        parsed_plans.append([name, start_min, int(play_time)])
        
    parsed_plans.sort(key=lambda x: x[1])
    
    answer = []
    stack = []
    
    for i in range(len(parsed_plans)):
        cn, cs, cp = parsed_plans[i]
        
        if i == len(parsed_plans) - 1:
            answer.append(cn)
            while stack:
                answer.append(stack.pop()[0])
            break
            
        nn, ns, np = parsed_plans[i + 1]
        
        if cs + cp > ns:
            rest = (cs + cp) - ns
            stack.append([cn, rest])
            
        else:
            answer.append(cn)
            tt = ns - (cs + cp)
            
            while stack and tt > 0:
                rn, rt = stack[-1]
                
                if rt <= tt:
                    tt -= rt
                    answer.append(rn)
                    stack.pop()
                else:
                    stack[-1][1] -= tt
                    tt = 0
                    
    return answer