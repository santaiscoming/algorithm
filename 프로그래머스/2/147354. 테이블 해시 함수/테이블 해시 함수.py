def solution(data, col, row_begin, row_end):
    data.sort(key=lambda x: (x[col - 1], -x[0]))
    
    ans = 0

    for i in range(row_begin, row_end + 1):
        row = data[i - 1]
        acc = sum(val % i for val in row)

        ans ^= acc

    return ans