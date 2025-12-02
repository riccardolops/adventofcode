import numpy as np

def ceck_words(matrix, i, j):
    internal = 0
    if i >= 3:
        if matrix[i-1][j] == "M" and matrix[i-2][j] == "A" and matrix[i-3][j] == "S":
            internal += 1
    if j >= 3:
        if matrix[i][j-1] == "M" and matrix[i][j-2] == "A" and matrix[i][j-3] == "S":
            internal += 1
    if i >= 3 and j >= 3:
        if matrix[i-1][j-1] == "M" and matrix[i-2][j-2] == "A" and matrix[i-3][j-3] == "S":
            internal += 1
    if i <= len(matrix)-4:
        if matrix[i+1][j] == "M" and matrix[i+2][j] == "A" and matrix[i+3][j] == "S":
            internal += 1
    if j <= len(matrix[0])-4:
        if matrix[i][j+1] == "M" and matrix[i][j+2] == "A" and matrix[i][j+3] == "S":
            internal += 1
    if i <= len(matrix)-4 and j <= len(matrix[0])-4:
        if matrix[i+1][j+1] == "M" and matrix[i+2][j+2] == "A" and matrix[i+3][j+3] == "S":
            internal += 1
    if i >= 3 and j <= len(matrix[0])-4:
        if matrix[i-1][j+1] == "M" and matrix[i-2][j+2] == "A" and matrix[i-3][j+3] == "S":
            internal += 1
    if i <= len(matrix)-4 and j >= 3:
        if matrix[i+1][j-1] == "M" and matrix[i+2][j-2] == "A" and matrix[i+3][j-3] == "S":
            internal += 1
    #print("i: " + str(i) + " j: " + str(j) + " count: " + str(internal)) 
    return internal

def ceck_words_mas(matrix, i, j):
    internal = 0
    if i > 0 and j > 0 and i <= len(matrix)-2 and j <= len(matrix[0])-2:
        #if (matrix[i-1][j] == "M" and matrix[i+1][j] == "S") or (matrix[i-1][j] == "S" and matrix[i+1][j] == "M"):
        #    if (matrix[i][j-1] == "M" and matrix[i][j+1] == "S") or (matrix[i][j-1] == "S" and matrix[i][j+1] == "M"):
        #        internal += 1
        if (matrix[i-1][j-1] == "M" and matrix[i+1][j+1] == "S") or (matrix[i-1][j-1] == "S" and matrix[i+1][j+1] == "M"):
            if (matrix[i-1][j+1] == "M" and matrix[i+1][j-1] == "S") or (matrix[i-1][j+1] == "S" and matrix[i+1][j-1] == "M"):
                internal += 1
    if internal == 2:
        #print("i: " + str(i) + " j: " + str(j) + " count: " + str(internal))
        internal = 0

    return internal

file = open("input4.txt", "r")

# read  the lines in a matrix of chars
matrix = [list(line.strip()) for line in file]
total_xmas = 0
total_mas = 0
for i in range(len(matrix)):
    for j in range(len(matrix[i])):
        char = matrix[i][j]
        if char == "X":
            n = ceck_words(matrix, i, j)
            total_xmas += n
        if char == "A":
            n = ceck_words_mas(matrix, i, j)
            total_mas += n

print(total_xmas)
print(total_mas)

