def solution(n, w, num):
    container = [[] for _ in range(w)]
    v = 0
    flag = True
    while v < n:
        if flag:
            for i in range(w):
                container[i].append(v)
                v += 1
                if v == n:
                    break
            flag = False
        else:
            for i in range(w - 1, -1, -1):
                container[i].append(v)
                v += 1
                if v == n:
                    break
            flag = True
        
    num = num - 1
    for arr in container:
        if num in arr:
            i = arr.index(num)
            return len(arr) - i