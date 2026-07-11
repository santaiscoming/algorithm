def solution(s):
    answer = []
    cnt = 0
    zero_cnt = 0
    while True:
        (once, zero) = parse(s)
        zero_cnt += zero
        cnt += 1
        s = str(bin(once)[2:])
        
        if once == 1:
            break

        
    return [cnt, zero_cnt]

def parse(s):
    once = len([c for c in s if c == '1'])
    return (once, len(s) - once)