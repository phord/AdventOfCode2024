#!/usr/bin/python3

'''
Parse the input into two groups: order rules and updates

Order rules is a map of pages and their required predecessors.

Updates is a list of pages for each update to be printed.
'''
def parse(inp):
    a,b = inp.split('\n\n')
    a = a.split('\n')
    b = b.split('\n')

    order = {}
    for p1, p2 in [(int(x), int(y)) for x,y in [line.split('|') for line in a]]:
        if p2 in order:
            order[p2].add(p1)
        else:
            order[p2] = set([p1])

    updates = []
    for pp in b:
        updates.append([int(x) for x in pp.split(',')])

    return order, updates


def valid(pp, order):
    done = set()
    # reverse the pages
    pp = pp[::-1]
    for p in pp:
        if p in done:
            return False
        if p in order:
            for inv in order[p]:
                if inv in done:
                    return False
        done.add(p)
    return True

def part1(order, updates):
    total = 0
    for pages in updates:
        if valid(pages, order):
            total += pages[len(pages)//2 ]
    print("Part1: ", total)

def try_fix(pp, order):
    todo = set(pp)
    fixed = []
    for p in pp:
        if p in fixed:
            continue
        if p in order:
            for inv in order[p]:
                if inv in todo:
                    fixed.append(inv)
        fixed.append(p)
        todo -= set(fixed)
    return fixed

def fix(pp, order):
    fixed = pp
    while not valid(fixed, order):
        fixed = try_fix(fixed, order)
    return fixed

def part2(order, updates):
    total = 0
    for pages in updates:
        if not valid(pages, order):
            fixed = fix(pages, order)
            assert set(fixed) == set(pages)
            assert(len(fixed) & 1 == 1)
            assert(valid(fixed, order))
            total += fixed[len(fixed)//2 ]

    print("Part2: ", total)

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


    part1( order, updates )

    part2( order, updates )

    with open('input5.txt', 'r') as f:
        input1 = f.read()
    order, updates = parse(input1)

    part1(order, updates)

    with open('input5.txt', 'r') as f:
        input2 = f.read()
    order, updates = parse(input2)
    part2( order, updates )

play()