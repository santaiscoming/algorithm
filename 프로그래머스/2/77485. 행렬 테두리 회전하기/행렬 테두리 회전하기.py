def solution(rows, columns, queries):
    ans = []
    
    mat = [[(r * columns) + c + 1 for c in range(columns)] for r in range(rows)]
    
    for x1, y1, x2, y2 in queries:
        x1, y1, x2, y2 = x1 - 1, y1 - 1, x2 - 1, y2 - 1
        
        temp = mat[x1][y1]
        min_v = temp
        
        for k in range(x1, x2):
            mat[k][y1] = mat[k+1][y1]
            min_v = min(min_v, mat[k][y1])
            
        for k in range(y1, y2):
            mat[x2][k] = mat[x2][k+1]
            min_v = min(min_v, mat[x2][k])
            
        for k in range(x2, x1, -1):
            mat[k][y2] = mat[k-1][y2]
            min_v = min(min_v, mat[k][y2])
            
        for k in range(y2, y1, -1):
            mat[x1][k] = mat[x1][k-1]
            min_v = min(min_v, mat[x1][k])
            
        mat[x1][y1+1] = temp
        
        ans.append(min_v)
        
    return ans