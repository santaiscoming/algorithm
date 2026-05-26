def solution(bandage, health, attacks):
    t, x, y = bandage
    hp = health
    prev_t = 0
    
    for curr_t, atk in attacks:
        gap = curr_t - prev_t - 1
        
        heal = (gap * x) + (gap // t) * y
        hp = min(health, hp + heal)
        
        hp -= atk
        
        if hp <= 0:
            return -1
            
        prev_t = curr_t
        
    return hp