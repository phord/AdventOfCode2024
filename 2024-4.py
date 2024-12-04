#!/usr/bin/python3

''' AOC 2024, day 3 '''

def find_at(inp, x, y, dx, dy):
    target="XMAS"
    height = len(inp)
    width = len(inp[0])
    tlen = len(target)
    for j in range(tlen):
        xx = x + j*dx
        yy = y + j*dy
        if xx < 0 or xx >= width:
            return False
        if yy < 0 or yy >= height:
            return False
        if inp[y+j*dy][x+j*dx] != target[j]:
            return False
    return True

def find_xmas(inp):
    count = 0
    height = len(inp)
    for y in range(height):
        for x in range(len(inp[y])):
            for (dx, dy) in [(0,1), (0,-1),   # horiz
                             (1,0), (-1,0),   # vert
                             (1,1), (1,-1),   # diag
                             (-1,1), (-1,-1),  # diag
                             ]:
                if find_at(inp, x, y, dx, dy):
                    count += 1
    return count

def find_x_at(inp, x, y, dx, dy):
    target="MAS"
    height = len(inp)
    width = len(inp[0])
    if x < 1 or x >= width-1:
        return False
    if y < 1 or y >= height-1:
        return False
    xx = x - dx
    yy = y - dy
    for j in range(len(target)):
        if inp[yy + j*dy][xx + j*dx] != target[j]:
            return False
    return True

def find_x_mas(inp):
    count = 0
    height = len(inp)
    for y in range(height):
        for x in range(len(inp[y])):
            if not find_x_at(inp, x, y, 1, 1) and not find_x_at(inp, x, y, -1, -1):
                continue
            if not find_x_at(inp, x, y, 1, -1) and not find_x_at(inp, x, y, -1, 1):
                continue
            count += 1

    return count


def part1(inp):
    print("Part1: ", find_xmas(inp))

def part2(inp):
    print("Part2: ", find_x_mas(inp))


input = '''MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX'''
input = input.split('\n')

part1( input )
part2( input )

with open('input4.txt', 'r') as f:
    input1 = f.read()
input1 = input1.split('\n')
part1(input1)

with open('input4.txt', 'r') as f:
    input2 = f.read()
input2 = input2.split('\n')
part2(input2)