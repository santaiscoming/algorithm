def solution(schedules, timelogs, startday):
    return sum(
        all(t <= s + (10 if s % 100 < 50 else 50)
            for j, t in enumerate(log) if (startday + j - 1) % 7 < 5)
        for s, log in zip(schedules, timelogs)
    )