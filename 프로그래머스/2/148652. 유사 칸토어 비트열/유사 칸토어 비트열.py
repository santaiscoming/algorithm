def solution(n, l, r):
    def count(to, depth):
        if to == 0:
            return 0
                    
        if depth == 0:
            return 1
        
        per_block_size = 5 ** (depth - 1)
        per_block_once = 4 ** (depth - 1)
        
        block_pos = to // per_block_size
        rest = to % per_block_size
        if block_pos > 2:
            ones = (block_pos - 1) * per_block_once
        else:
            ones = block_pos * per_block_once
        
        if block_pos % 5 == 2:
            return ones

        return ones + count(rest, depth - 1)
    
    return count(r, n) - count(l - 1, n)