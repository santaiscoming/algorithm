from itertools import combinations, product
from bisect import bisect_left

def solution(dice):
    n = len(dice)
    indices = range(n)
    
    max_wins = -1
    best_combination = []
    

    for a_indices in combinations(indices, n // 2):
        b_indices = [i for i in indices if i not in a_indices]
        
        a_selected = [dice[i] for i in a_indices]
        b_selected = [dice[i] for i in b_indices]

        
        a_sums = list(map(sum, product(*a_selected)))
        b_sums = list(map(sum, product(*b_selected)))
        
        b_sums.sort()
        current_wins = 0

        for a_score in a_sums:
            wins = bisect_left(b_sums, a_score)
            current_wins += wins
            
        if current_wins > max_wins:
            max_wins = current_wins
            best_combination = a_indices
            
    return sorted([x + 1 for x in best_combination])