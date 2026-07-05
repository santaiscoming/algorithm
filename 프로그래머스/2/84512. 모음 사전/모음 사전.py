def solution(word):
    a = ['A', 'E', 'I', 'O', 'U']
    combi = []
    
    def recur(depth, string):
        if depth == 5:
            return
        
        for i in range(5):
            string.append(a[i])
            combi.append("".join(string))
            recur(depth + 1, string[:])
            string.pop()
            
    recur(0, [])
    for i, v in enumerate(combi):
        if v == word:
            return i + 1