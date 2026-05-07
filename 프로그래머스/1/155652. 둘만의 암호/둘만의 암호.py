def solution(s, skip, index):
    ans = ''
    
    for c in s:
        start = ord(c) - 97
        move = 0
        i = 0
        while i < index:
            move += 1
            next = chr(((start + move) % 26) + 97)
            if not next in skip:
                i += 1
            
        next = chr(((start + move) % 26) + 97)
        ans += next
                
    return ans

    