def solution(id_list, report, k):
    report = set(report)
    
    cnt = {v: 0 for v in id_list}
    users = {v: [] for v in id_list}
    
    for r in report:
        _from, t = r.split()
        users[_from].append(t)
        cnt[t] += 1
        
    banned = {user for user, count in cnt.items() if count >= k}
    return [sum(1 for t in users[id] if t in banned) for id in id_list]
