def solution(message, spoiler_ranges):
    non_spo_words = get_non_spo_words(message, spoiler_ranges)
    answer = 0
    spo_words = set()
    word = ""
    is_spo = False
    for i, v in enumerate(message):
        if v == " ":
            if is_important(word, non_spo_words, spo_words, is_spo):
                answer += 1
                spo_words.add(word)
            word = ""
            is_spo = False
            continue
        for s, e in spoiler_ranges:
            if s <= i <= e:
                is_spo = True
        word += v
        
        if i == len(message) - 1:
            if is_important(word, non_spo_words, spo_words, is_spo):
                answer += 1
                spo_words.add(word)
        
    
    return answer

def get_non_spo_words(message, spoiler_ranges):
    ret = set()    
    word = ""
    is_spo = False
    for i, c in enumerate(message):
        if c == ' ':
            if not is_spo:
                ret.add(word)
            word = ""
            is_spo = False
            continue
            
        word += c
        
        for s, e in spoiler_ranges:
            if s <= i <= e:
                is_spo = True
        
        if i == len(message) - 1:
            if not is_spo:
                ret.add(word)
    return ret

def is_important(word, non_spo, spos, is_spo):
    if word in non_spo or word in spos or not is_spo:
        return False
    
    return True