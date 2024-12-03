#!/usr/bin/python3

''' AOC 2024, day 3 '''

def decode_token(prog):
    '''
    Extracts the next token at the start of string and returns the
    token and the remainder of the string
    Recognized tokens:
        mul
        (
        integer
        ,
        )
    Anything else is garbage.  It returns None for the token, and the remainder of the string.
    '''

    keywords = ["mul", "(", ",", ")", "don't", "do"]

    token, remainder = decode_int(prog)
    if token is not None:
        return token, remainder

    for k in keywords:
        if prog.startswith(k):
            return k, prog[len(k):]
    return None, prog[1:]

def is_integer(prog):
    '''
    Returns true if the first character of prog is a digit
    '''
    return prog[0] in "0123456789"

def decode_patt(prog, match):
    '''
    Returns a tuple of the tokens that match the pattern.
    If the pattern does not match, returns None, prog.
    '''
    catch = []

    remainder = prog
    for m in match:
        token, remainder = decode_token(remainder)
        catch.append(token)
        if token is None:
            return None, prog
        if m == "integer":
            if not is_integer(token):
                return None, prog
        elif token != m:
            return None, prog
    return (catch, remainder)


def decode_mul(prog):
    '''
    Returns the result of the mul operation and the remainder of the string.
    If the string does not start with a mul operation, returns None, prog.
    '''
    match = ["mul", "(", "integer", ",", "integer", ")"]

    catch, remainder = decode_patt(prog, match)
    if catch is None:
        return None, prog
    return (int(catch[2]) * int(catch[4]), remainder)

def decode_do(prog):
    '''
    Returns the result of the do operation and the remainder of the string.
    If the string does not start with a do operation, returns None, prog.
    '''
    match = ["do", "(", ")"]

    catch, remainder = decode_patt(prog, match)
    if catch is None:
        return None, prog
    return (True, remainder)


def decode_dont(prog):
    '''
    Returns the result of the don't operation and the remainder of the string.
    If the string does not start with a don't operation, returns None, prog.
    '''
    match = ["don't", "(", ")"]

    catch, remainder = decode_patt(prog, match)

    if catch is None:
        return None, prog
    return (True, remainder)


def decode_int(prog):
    '''
    Returns the integer at the start of the string and the remainder of the string.
    If the string does not start with an integer, returns None, prog.
    '''
    val = ""
    while prog[0] in "0123456789":
        val += prog[0]
        prog = prog[1:]
    if val == "":
        return None, prog
    return (val, prog)

def part1(inp):
    ''' part 1 '''

    total = 0
    while inp:
        token, inp = decode_mul(inp)
        if token is not None:
            total += token
        else:
            inp = inp[1:]

    print(f"Part1: {total}")

def part2(inp):
    ''' part 2 '''

    total = 0
    enabled = True
    while inp:
        token, inp = decode_mul(inp)
        if token is not None:
            if enabled:
                total += token
            continue
        token, inp = decode_do(inp)
        if token is not None:
            enabled = True
            continue
        token, inp = decode_dont(inp)
        if token is not None:
            enabled = False
            continue
        inp = inp[1:]

    print(f"Part2: {total}")

input1= "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
with open('input3.txt', 'r') as f:
    input1 = f.read()
part1(input1)

input2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
with open('input3.txt', 'r') as f:
    input2 = f.read()
part2(input2)