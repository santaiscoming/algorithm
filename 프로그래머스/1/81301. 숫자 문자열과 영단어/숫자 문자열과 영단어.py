def solution(s):
    Map = {
        "zero": 0,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
    }
    
    ans = ""
    tmp = ""
    for c in s:
        if c.isdigit():
            ans += c
        else:
            tmp += c
            
        if tmp in Map:
            ans += str(Map[tmp])
            tmp = ""

    return int(ans)