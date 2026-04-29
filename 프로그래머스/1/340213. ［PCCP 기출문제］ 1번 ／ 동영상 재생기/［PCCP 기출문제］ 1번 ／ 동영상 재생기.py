def solution(video_len, pos, op_start, op_end, commands):
    answer = ''
    [video_len, pos, op_start, op_end] = list(map(parse, [video_len, pos, op_start, op_end]))
    
    if op_start <= pos <= op_end: 
        pos = op_end
    
    for c in commands:
        if c == 'next':
            pos += 10
            if pos > video_len:
                pos = video_len
        else:
            pos -= 10
            if pos < 0:
                pos = 0
                
        if op_start <= pos <= op_end: 
            pos = op_end
            
    return f"{(pos // 60):02d}:{(pos % 60):02d}"

def parse(time):
    [mm, ss] = list(map(int, time.split(":")));
    return mm * 60 + ss
    
    