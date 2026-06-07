def solution(data, ext, val_ext, sort_by):
    hashmap = {
        "code": 0,
        "date": 1,
        "maximum": 2,
        "remain": 3
    }
    
    fk = hashmap[sort_by]
    data.sort(key=lambda x: x[fk])
    ek = hashmap[ext]
    data = list(filter(lambda x: x[ek] < val_ext, data))
    
    return data