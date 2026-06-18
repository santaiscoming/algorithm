def solution(book_time):
    book_time = list(map(lambda x: parse(x[0], x[1]), book_time))
    
    events = []
    for s, e in book_time:
        events.append((s, 1))
        events.append((e, -1))
    
    ans = 0
    curr = 0
    events.sort(key=lambda x: (x[0], x[1]))
    print(events)
    for _, e in events:
        curr += e
        ans = max(ans, curr)
        
    return ans

def parse(s, e):
    sh, sm = s.split(":")
    eh, em = e.split(":")
    
    return (int(sh) * 3600 + int(sm) * 60, int(eh) * 3600 + (int(em) + 10) * 60)