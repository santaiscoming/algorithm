import math

def solution(arrayA, arrayB):
    answer = 0
    
    gcdA = arrayA[0]
    for num in arrayA[1:]:
        gcdA = math.gcd(gcdA, num)
        
    gcdB = arrayB[0]
    for num in arrayB[1:]:
        gcdB = math.gcd(gcdB, num)
        
    if not any(num % gcdA == 0 for num in arrayB):
        answer = max(answer, gcdA)
        
    if not any(num % gcdB == 0 for num in arrayA):
        answer = max(answer, gcdB)
        
    return answer