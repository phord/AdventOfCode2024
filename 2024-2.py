#!/usr/bin/python3

input = '''7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9'''

# read whole text file from 2024-2.txt into input
with open('2024-2.txt', 'r') as f:
    input = f.read()

def safe(level):
    x = zip(level, level[1:])
    x = [ b-a for (a,b) in x]

    desc = set([-4 < i and i < 0 for i in x])
    asc = set([0 < i and i < 4 for i in x])
    return (len(desc) == 1 and True in desc) or (len(asc) == 1 and True in asc)

def part1():
    global input

    levels = [list(map(int, line.split())) for line in input.split('\n')]

    count = 0
    for l in levels:
        if safe(l):
            count += 1
    print(f"Part1: {count}")

def part2():
    global input

    levels = [list(map(int, line.split())) for line in input.split('\n')]

    count = 0
    for l in levels:
        if safe(l):
            count += 1
        else:
            for i in range(len(l)):
                # copy level and remove ith item
                level = l.copy()
                level.pop(i)
                if safe(level):
                    count += 1
                    break

    print(f"Part2: {count}")

part1()
part2()