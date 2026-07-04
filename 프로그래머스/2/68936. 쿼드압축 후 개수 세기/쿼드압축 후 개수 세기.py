def solution(arr):
    ans = [0, 0]
    
    def recur(x, y, n):
        start_val = arr[x][y]
        
        for i in range(x, x + n):
            for j in range(y, y + n):
                if arr[i][j] != start_val:
                    next_n = n // 2
                    recur(x, y, next_n)
                    recur(x, y + next_n, next_n)
                    recur(x + next_n, y, next_n)
                    recur(x + next_n, y + next_n, next_n)
                    return

        ans[start_val] += 1

    recur(0, 0, len(arr))
    
    return ans