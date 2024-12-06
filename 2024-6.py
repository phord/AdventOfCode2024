#!/usr/bin/python3

dirs = [(0,-1), (1,0), (0,1), (-1,0)]

class Map:
    def __init__(self, inp):
        map = inp.split('\n')

        self.height = len(map)
        self.width = len(map[0])

        cells = {(x, y): cell
                    for y, line in enumerate(map)
                    for x, cell in enumerate(line)
                }

        self.pos = [pos for pos in cells if cells[pos] == '^'][0]
        self.walls = frozenset(pos for pos in cells if cells[pos] == '#')
        self.guard = 0
        self.start = (self.guard, self.pos)
        self.added = (-1,-1)

    def reset(self):
        self.guard, self.pos = self.start

    def valid(self):
        x,y = self.pos
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def turn_right(self):
        self.guard = (self.guard + 1) % 4

    def next_pos(self):
        x,y = self.pos
        dx,dy = dirs[self.guard]
        return (x + dx, y + dy)

    def blocked(self, pos):
        return pos in self.walls or pos == self.added

    def move(self):
        pos = self.next_pos()
        while self.blocked(pos):
            self.turn_right()
            pos = self.next_pos()
        self.pos = pos

    def path(self):
        while self.valid():
            yield self.pos
            self.move()

    def loops(self):
        loop = set()
        while self.valid():
            state = (self.pos, self.guard)
            if state in loop:
                return True
            loop.add(state)
            self.move()
        return False

    def add(self, pos):
        self.added = pos

def part2(map):
    count = 0
    map.reset()
    possible = frozenset([p for p in map.path()])
    map.reset()
    possible = possible - frozenset([map.pos])
    for pos in possible:
        map.add(pos)
        if map.loops():
            count += 1
        map.reset()
    print("Part2: ", count)

def part1(map):
    path = set([p for p in map.path()])
    print("Part1: ", len(path))

def sample():
    return '''....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...'''

def play():
    map = Map(sample())

    part1( map )
    part2( map )

    with open('input6.txt', 'r') as f:
        input1 = f.read()
    map = Map(input1)

    part1(map)
    part2(map)

play()