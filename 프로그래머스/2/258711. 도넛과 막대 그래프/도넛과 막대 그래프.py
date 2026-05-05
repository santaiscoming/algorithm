def solution(edges):
    degree = {}
    
    for a, b in edges:
        if a not in degree: degree[a] = [0, 0]
        if b not in degree: degree[b] = [0, 0]
        
        degree[a][1] += 1  
        degree[b][0] += 1  
        
    center_node = 0
    bar_cnt = 0
    eight_cnt = 0
    
    for node, (in_d, out_d) in degree.items():
        if in_d == 0 and out_d >= 2:
            center_node = node
    
    total_graphs = degree[center_node][1]

    for node, (in_d, out_d) in degree.items():
        if node == center_node:
            continue
                    
        if out_d == 0:
            bar_cnt += 1
        elif in_d >= 2 and out_d == 2:
            eight_cnt += 1
            
    donut_cnt = total_graphs - bar_cnt - eight_cnt
    
    return [center_node, donut_cnt, bar_cnt, eight_cnt]