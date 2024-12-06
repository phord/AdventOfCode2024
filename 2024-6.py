#!/usr/bin/python3

class Map:

    def __init__(self, inp):
        map = inp.split('\n')

        self.height = len(map)
        self.width = len(map[0])

        self.walls = set()
        for y, line in enumerate(map):
            for x, cell in enumerate(line):
                if cell == '#':
                    self.walls.add( (x,y) )
                elif cell in '^v<>':
                    self.guard = cell
                    self.pos = (x,y)
                    self.start = (self.guard, self.pos)

    def reset(self):
        self.guard, self.pos = self.start

    def valid(self):
        x,y = self.pos
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def turn_right(self):
        if self.guard == '^':
            self.guard = '>'
        elif self.guard == '>':
            self.guard = 'v'
        elif self.guard == 'v':
            self.guard = '<'
        elif self.guard == '<':
            self.guard = '^'
        else:
            assert(False)

    def next_pos(self):
        x,y = self.pos
        if self.guard == '^':
            y -= 1
        elif self.guard == 'v':
            y += 1
        elif self.guard == '<':
            x -= 1
        elif self.guard == '>':
            x += 1
        return (x,y)


    def move(self):
        pos = self.next_pos()
        while pos in self.walls:
            self.turn_right()
            pos = self.next_pos()

        self.pos = pos


    def __str__(self):
        return f"Map(walls={self.walls}, guard={self.guard}, pos={self.pos}, height={self.height}, width={self.width})"

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

    def open_cells(self):
        for y in range(self.height):
            for x in range(self.width):
                if (x,y) not in self.walls and (x,y) != self.pos:
                    yield (x,y)

    def add(self, pos):
        self.walls.add(pos)

    def remove(self, pos):
        self.walls.remove(pos)


def part2(map):
    count = 0
    map.reset()
    possible = set([p for p in map.path()])
    map.reset()
    possible.remove(map.pos)
    for pos in possible:
        # print(f"Obstruction {pos}")
        map.add(pos)
        if map.loops():
            count += 1
        map.remove(pos)
        map.reset()
    print("Part2: ", count)

def solve(map):
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


    solve( map )
    part2( map )

    with open('input6.txt', 'r') as f:
        input1 = f.read()
    map = Map(input1)

    solve(map)
    part2(map)

play()