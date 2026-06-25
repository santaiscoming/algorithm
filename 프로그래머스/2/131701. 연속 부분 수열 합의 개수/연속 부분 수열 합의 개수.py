def solution(el):
    n = len(el)
    freq = set()
    
    for s in range(n):
        acc = 0
        
        for cnt in range(n):
            acc += el[(s + cnt) % n]
            freq.add(acc)
            
    return len(freq)