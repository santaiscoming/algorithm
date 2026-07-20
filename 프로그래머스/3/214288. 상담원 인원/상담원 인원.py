import heapq

def solution(k, n, reqs):
    by_type = [[] for _ in range(k + 1)]
    for a, b, c in reqs:
        by_type[c].append((a, b))
        
    max_m = n - k + 1
    times = [[0] * (max_m + 1) for _ in range(k + 1)]
    for t in range(1, k + 1):
        for m in range(1, max_m + 1):
            times[t][m] = wait_time(by_type[t], m)
            
    m_cnt = [1] * (k + 1)
    for _ in range(n - k):
        best_t, max_diff = 0, -1
        for t in range(1, k + 1):
            diff = times[t][m_cnt[t]] - times[t][m_cnt[t] + 1]
            if diff > max_diff:
                max_diff = diff
                best_t = t
        if max_diff <= 0:
            break
        m_cnt[best_t] += 1
        
    return sum(times[t][m_cnt[t]] for t in range(1, k + 1))

def wait_time(reqs, mentors):
    if not reqs:
        return 0
    heap = []
    wait = 0
    for start, dur in reqs:
        if len(heap) < mentors:
            heapq.heappush(heap, start + dur)
        else:
            end = heapq.heappop(heap)
            if end > start:
                wait += end - start
                heapq.heappush(heap, end + dur)
            else:
                heapq.heappush(heap, start + dur)
    return wait
