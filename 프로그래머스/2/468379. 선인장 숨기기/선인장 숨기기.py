from collections import deque

def solution(m, n, h, w, drops):
    INF = len(drops) + 1
    grid = [[INF] * n for _ in range(m)]
    
    for i, pos in enumerate(drops):
        r, col = pos
        grid[r][col] = i + 1

    def window(arr, size):
        res = []
        q = deque()
        for i, val in enumerate(arr):
            if q and q[0] < i - size + 1:
                q.popleft()
                
            while q and arr[q[-1]] >= val:
                q.pop()
                
            q.append(i)
            
            if i >= size - 1:
                res.append(arr[q[0]])
                
        return res

    row_mins = [window(grid[r], w) for r in range(m)]

    valid_rows = m - h + 1
    valid_cols = n - w + 1
    rect_mins = [[0] * valid_cols for _ in range(valid_rows)]
    
    for col in range(valid_cols):
        col_data = [row_mins[r][col] for r in range(m)]
        col_mins = window(col_data, h)
        
        for r, min_val in enumerate(col_mins):
            rect_mins[r][col] = min_val

    max_safe_time = -1
    answer = [0, 0]
    
    for r in range(valid_rows):
        for col in range(valid_cols):
            if rect_mins[r][col] > max_safe_time:
                max_safe_time = rect_mins[r][col]
                answer = [r, col]
                
    return answer