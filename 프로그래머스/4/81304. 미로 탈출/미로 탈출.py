import heapq

def solution(n, start, end, roads, traps):
    INF = float('inf')
    trap_idx = {trap: i for i, trap in enumerate(traps)}
    
    graph = {i: [] for i in range(1, n + 1)}
    for u, v, w in roads:
        graph[u].append((v, w, False))
        graph[v].append((u, w, True))  
        
    dist = [[INF] * (1 << len(traps)) for _ in range(n + 1)]
    dist[start][0] = 0
    
    pq = []
    heapq.heappush(pq, (0, start, 0))
    
    while pq:
        cost, u, state = heapq.heappop(pq)
        
        if u == end:
            return cost
            
        if cost > dist[u][state]:
            continue
            
        u_is_trap = u in trap_idx
        u_flipped = bool(state & (1 << trap_idx[u])) if u_is_trap else False
        
        for v, w, is_rev in graph[u]:
            v_is_trap = v in trap_idx
            v_flipped = bool(state & (1 << trap_idx[v])) if v_is_trap else False
            
            # [핵심] 두 노드의 뒤집힘 상태를 XOR 연산하여 현재 간선의 사용 가능 여부 파악
            # (u_flipped ^ v_flipped) 와 간선의 원래 역방향 여부가 일치해야만 이동 가능
            if (u_flipped ^ v_flipped) == is_rev:
                
                # 다음 상태 계산 (다음 노드가 함정이라면 비트를 토글(XOR))
                next_state = state
                if v_is_trap:
                    next_state ^= (1 << trap_idx[v])
                    
                next_cost = cost + w
                if cost + w < dist[v][next_state]:
                    dist[v][next_state] = next_cost
                    heapq.heappush(pq, (next_cost, v, next_state))