def solution(numbers):
    n = len(numbers)
    
    ans = [-1] * n
    s = []
    for i in range(len(numbers)):
        while s and numbers[s[-1]] < numbers[i]:
            ans[s[-1]] = numbers[i]
            s.pop()
            
        s.append(i)
    
    return ans