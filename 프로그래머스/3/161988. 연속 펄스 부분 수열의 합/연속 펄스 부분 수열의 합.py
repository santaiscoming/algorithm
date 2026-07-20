def solution(sequence):
    prefix = [0]
    
    v = 1
    for num in sequence:
        prefix.append(prefix[-1] + num * v)
        v *= -1
        
    return max(prefix) - min(prefix)