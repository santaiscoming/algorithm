from collections import deque

def solution(rows, columns, queries):
    for i in range(5, 1, -1):
        print(i)
    ans = []
    mat = [[0] * columns for _ in range(rows)]
    for r in range(rows):
        for c in range(columns):
            mat[r][c] = (columns * r) + (c + 1)
    

    for r1, c1, r2, c2 in queries:
        r1, c1, r2, c2 = r1 - 1, c1 - 1, r2 - 1, c2 - 1
        tmp = deque()
        
        for c in range(c1, c2): tmp.append(mat[r1][c])
        for r in range(r1, r2): tmp.append(mat[r][c2])
        for c in range(c2, c1, -1): tmp.append(mat[r2][c])
        for r in range(r2, r1, -1): tmp.append(mat[r][c1])
        
        ans.append(min(tmp))
        tmp.rotate(1)
        
        i = 0
        for c in range(c1, c2): mat[r1][c] = tmp[i]; i += 1
        for r in range(r1, r2): mat[r][c2] = tmp[i]; i += 1
        for c in range(c2, c1, -1): mat[r2][c] = tmp[i]; i += 1
        for r in range(r2, r1, -1): mat[r][c1] = tmp[i]; i += 1
        
    return ans