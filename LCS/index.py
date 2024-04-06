X = "ABCBDAB"
Y = "BDCABA"


def create_table(word_1, word_2):
    table = [[0] * (len(word_1) + 1) for _ in range(len(word_2) + 1)]

    return table


# 점화식 : LCS(i, j) : LCS(i - 1, j - 1) + 1, or max(LCS(i - 1, j), LCS(i, j - 1))
def LCS(word_1, word_2):
    X_axis_word = word_1
    Y_axis_word = word_2
    table = create_table(X_axis_word, Y_axis_word)

    for y in range(1, len(Y_axis_word) + 1):
        for x in range(1, len(X_axis_word) + 1):
            #  (0, 0)은 둘다 선택안한것을 의미한다
            #  (1, 1) 부터 word 마킹 시작 -> LCS(i, j) = word[i - 1], word[j - 1]
            x_char = X_axis_word[x - 1]
            y_char = Y_axis_word[y - 1]

            # 두 단어가 같은 경우 LCS(i - 1, j - 1) + 1
            print(x_char, y_char)
            if x_char == y_char:
                prev_lcs = table[y - 1][x - 1]
                table[y][x] = prev_lcs + 1

            # 두 단어가 다른경우
            else:
                top = table[y - 1][x]  # LCS(i - 1, j)
                left = table[y][x - 1]  # LCS(i, j - 1)

                table[y][x] = max(top, left)

    return table


result = LCS(X, Y)

for row in result:
    print(row)
