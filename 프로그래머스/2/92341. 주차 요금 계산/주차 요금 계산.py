import math
from collections import defaultdict

def solution(fees, records):
    dt, df, ut, uf = fees
    park, total = {}, defaultdict(int)
    
    for r in records:
        t, c, s = r.split()
        m = parse(t)
        if s == "IN":
            park[c] = m
        else:
            total[c] += m - park[c]
            del park[c]
            
    for c, m in park.items():
        total[c] += 1439 - m
        
    ans = []
    for c in sorted(total.keys()):
        t = total[c]
        if t <= dt:
            ans.append(df)
        else:
            ans.append(df + math.ceil((t - dt) / ut) * uf)
            
    return ans


def parse(t):
    h, m = map(int, t.split(':'))
    return h * 60 + m
