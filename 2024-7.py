#!/usr/bin/python3

class Game:
    def __init__(self, inp):
        self.rules = []

        for test in inp.split('\n'):
            a,b = test.split(':')
            value = int(a)
            opands = [int(x) for x in b.split()]
            self.rules.append((value, opands))

def eval(test, part2):
    if len(test) == 1:
        yield test[0]
        return

    for ans in eval(test[:-1], part2):
        yield ans * test[-1]
        yield ans + test[-1]
        if part2:
            yield int(str(ans) + str(test[-1]))


def run(game, part2):
    total = 0
    for ans, opands in game.rules:
        if ans in eval(opands, part2):
            total += ans
    return total


def part1(game, part2=False):
    print("Part1: ", run(game, False))

def part2(game):
    print("Part2: ", run(game, True))


def sample():
    return '''190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20'''

def play():
    game = Game(sample())

    part1( game )
    part2( game )

    with open('input7.txt', 'r') as f:
        input1 = f.read()
    game = Game(input1)

    part1(game)
    part2(game)

play()