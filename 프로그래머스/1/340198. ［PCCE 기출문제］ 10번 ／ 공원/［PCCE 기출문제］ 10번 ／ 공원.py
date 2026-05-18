def solution(mats, park):
    h = len(park)
    w = len(park[0])

    mats.sort(reverse=True)

    def can_place(r, c, size):
        for i in range(r, r + size):
            for j in range(c, c + size):
                if park[i][j] != "-1":
                    return False

        return True

    for size in mats:
        if size > h or size > w:
            continue

        for r in range(h - size + 1):
            for c in range(w - size + 1):
                if can_place(r, c, size):
                    return size

    return -1