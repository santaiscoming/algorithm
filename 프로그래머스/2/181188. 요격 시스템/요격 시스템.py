def solution(targets):
    targets.sort(key=lambda x: x[1])
    
    answer = 0
    prev_e = 0
    for s, e in targets:
        if s >= prev_e:
            answer += 1
            prev_e = e
            
    return answer