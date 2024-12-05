#!/usr/bin/python3

'''
Parse the input into two groups: order rules and updates

Order rules is a map of pages and their required predecessors.

Updates is a list of pages for each update to be printed.
'''
def parse(inp):
    rules,updates = (x.split('\n') for x in inp.split('\n\n'))

    order = {}
    edges = [(int(x), int(y)) for x,y in [line.split('|') for line in rules]]
    # graph = { key: set(y for x,y in edges if x == key) for key in set(x for x,y in edges)}
    for p1, p2 in edges:
        if p2 in order:
            order[p2].add(p1)
        else:
            order[p2] = set([p1])

    updates = [[int(x) for x in pp.split(',')] for pp in updates]

    return order, updates

def try_fix(pp, order):
    todo = set(pp)
    fixed = []
    for p in pp:
        if p in todo:
            precedents = order[p] if p in order else set()
            fixed.extend(list(precedents.intersection(todo)))
            todo -= precedents
            fixed.append(p)
            todo.remove(p)
    return fixed

def fix(pp, order):
    fixed = pp
    while True:
        sorted = try_fix(fixed, order)
        if fixed == sorted:
            break
        fixed = sorted
    return fixed

def solve(order, updates):
    part1 = 0
    part2 = 0
    for pages in updates:
        sorted = fix(pages, order)
        if sorted == pages:
            part1 += pages[len(pages)//2]
        else:
            part2 += sorted[len(sorted)//2]

    print("Part1: ", part1)
    print("Part2: ", part2)

def sample():
    return '''47|53
    97|13
    97|61
    97|47
    75|29
    61|13
    75|53
    29|13
    97|29
    53|29
    61|53
    97|53
    61|29
    47|13
    75|47
    97|75
    47|61
    75|61
    47|29
    75|13
    53|13

    75,47,61,53,29
    97,61,53,29,13
    75,29,13
    75,97,47,61,53
    61,13,29
    97,13,75,29,47'''

def play():
    order, updates = parse(sample())
    # print(order)
    # print(updates)


    solve( order, updates )

    with open('input5.txt', 'r') as f:
        input1 = f.read()
    order, updates = parse(input1)

    solve(order, updates)

play()