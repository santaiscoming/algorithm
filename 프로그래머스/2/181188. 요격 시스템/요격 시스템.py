def solution(targets):
    targets.sort(key=lambda x: x[1])
    answer = 0
    end = 0
    
    for s, e in targets:
        if s >= end:
            answer += 1
            end = e
            
    return answer