import operator
import os
fileName = "input"


def decode_instructions(instructions):
    instruction_list = []
    character_state = instructions[0].isdigit()
    current_instruction = ''
    for character in instructions:
        if character.isdigit() == character_state:
            current_instruction += character
            continue
        character_state = character.isdigit()
        instruction_list.append(current_instruction)
        current_instruction = character
    for i in range(len(instruction_list)):
        if instruction_list[i].isdigit():
            instruction_list[i] = int(instruction_list[i])
    return instruction_list


def linkBorderSectors(size, sector1, border1, cw1, sector2, border2, cw2):
    pass


def linkBorder(origin1, direction1, origin2, direction2, length, input_direction, output_direction, borderLink,puzzle_map=None):
    pos1 = origin1
    pos2 = origin2
    inp = []
    outp = []
    for i in range(length):
        inp.append(pos1)
        outp.append(pos2)
        borderLink[(pos1, input_direction)] = (pos2, output_direction)
        pos1 = tuple(map(operator.add, pos1, direction1))
        pos2 = tuple(map(operator.add, pos2, direction2))
    if puzzle_map != None:
        printMap(puzzle_map, (0,0),inp,outp)
        input()

def printMap(puzzle_map, current_position, inp=None, outp=None, path=None):
    for line_number in range(-1, len(puzzle_map)+1):
        for column_number in range(-1,max(map(len, puzzle_map))+1):
            if (line_number,column_number) == current_position:
                print('@', end="")
                continue
            if path and (line_number,column_number) in path:
                print('X', end="")
                continue
            if inp and (line_number,column_number) in inp:
                if ((0 <= line_number < len(puzzle_map)) and
                   (0 <= column_number < len(puzzle_map[line_number])) and
                   (puzzle_map[line_number][column_number] != ' ')):
                    raise ValueError
                print('I', end="")
                continue
            if outp and (line_number,column_number) in outp:
                if not ((0 <= line_number < len(puzzle_map)) and
                   (0 <= column_number < len(puzzle_map[line_number])) and
                   (puzzle_map[line_number][column_number] != ' ')):
                    print("line_number", line_number)
                    print("column_number", column_number)
                    print("len(puzzle_map)", len(puzzle_map))
                    print("len(puzzle_map[line_number])", len(puzzle_map[line_number]))
                    print("puzzle_map[line_number][column_number]", puzzle_map[line_number][column_number])
                    raise ValueError
                print('O', end="")
                continue
            if line_number in list(range(len(puzzle_map))):
                if column_number in list(range(len(puzzle_map[line_number]))):
                    print(puzzle_map[line_number][column_number], end="")
                    continue
            print(' ', end='')
        print('',end="\n")

puzzle_map = []

with open(file=fileName, mode="r") as f:
    for line in f:
        line_content = line.strip('\r\n')
        if line_content == '':
            break
        puzzle_map.append(line_content)
    instructions = f.readline()

instruction_list = decode_instructions(instructions)

borderLink = {}

if fileName == "test_input":
    current_direction = (0, 1)
    current_position = (0, 8)

    linkBorder((0, 12), (1, 0), (0, 8), (1, 0), 4, (0, 1), (0, 1), borderLink)
    linkBorder((0, 7), (1, 0), (0, 11), (1, 0), 4, (0, -1), (0, -1), borderLink)

    linkBorder((4, 12), (1, 0), (4, 0), (1, 0), 4, (0, 1), (0, 1), borderLink)
    linkBorder((4, -1), (1, 0), (4, 11), (1, 0), 4, (0, -1), (0, -1), borderLink)

    linkBorder((8, 16), (1, 0), (8, 8), (1, 0), 4, (0, 1), (0, 1), borderLink)
    linkBorder((8, 7), (1, 0), (8, 15), (1, 0), 4, (0, -1), (0, -1), borderLink)


    linkBorder((8, 0), (0, 1), (4, 0), (0, 1), 4, (1, 0), (1, 0), borderLink)
    linkBorder((3, 0), (0, 1), (7, 0), (0, 1), 4, (-1, 0), (-1, 0), borderLink)

    linkBorder((8, 4), (0, 1), (4, 4), (0, 1), 4, (1, 0), (1, 0), borderLink)
    linkBorder((3, 4), (0, 1), (7, 4), (0, 1), 4, (-1, 0), (-1, 0), borderLink)

    linkBorder((12, 8), (0, 1), (0, 8), (0, 1), 4, (1, 0), (1, 0), borderLink)
    linkBorder((-1, 8), (0, 1), (11, 8), (0, 1), 4, (-1, 0), (-1, 0), borderLink)

    linkBorder((12, 12), (0, 1), (8, 12), (0, 1), 4, (1, 0), (1, 0), borderLink)
    linkBorder((7, 12), (0, 1), (11, 12), (0, 1), 4, (-1, 0), (-1, 0), borderLink)

if fileName == "input":
    current_direction = (0, 1)
    current_position = (0, 50)

    linkBorder((0, 150), (1, 0), (0, 50), (1, 0), 50, (0, 1), (0, 1), borderLink)
    linkBorder((0, 49), (1, 0), (0, 149), (1, 0), 50, (0, -1), (0, -1), borderLink)


    linkBorder((50, 100), (1, 0), (50, 50), (1, 0), 50, (0, 1), (0, 1), borderLink)
    linkBorder((50, 49), (1, 0), (50, 99), (1, 0), 50, (0, -1), (0, -1), borderLink)


    linkBorder((100, 100), (1, 0), (100, 0), (1, 0), 50, (0, 1), (0, 1), borderLink)
    linkBorder((100, -1), (1, 0), (100, 99), (1, 0), 50, (0, -1), (0, -1), borderLink)


    linkBorder((150, 50), (1, 0), (150, 0), (1, 0), 50, (0, 1), (0, 1), borderLink)
    linkBorder((150, -1), (1, 0), (150, 49), (1, 0), 50, (0, -1), (0, -1), borderLink)


    linkBorder((200, 0), (0, 1), (100, 0), (0, 1), 50, (1, 0), (1, 0), borderLink)
    linkBorder((99, 0), (0, 1), (199, 0), (0, 1), 50, (-1, 0), (-1, 0), borderLink)


    linkBorder((150, 50), (0, 1), (0, 50), (0, 1), 50, (1, 0), (1, 0), borderLink)
    linkBorder((-1, 50), (0, 1), (149, 50), (0, 1), 50, (-1, 0), (-1, 0), borderLink)

    linkBorder((50, 100), (0, 1), (0, 100), (0, 1), 50, (1, 0), (1, 0), borderLink)
    linkBorder((-1, 100), (0, 1), (49, 100), (0, 1), 50, (-1, 0), (-1, 0), borderLink)



for instruction in instruction_list:
    if type(instruction) is type(int(1)):
        for i in range(instruction):
            next_position = tuple(map(operator.add, current_position, current_direction))
            prev_direction = current_direction
            if (next_position, current_direction) in borderLink.keys():
                next_position, current_direction = borderLink[(next_position, current_direction)]
            if puzzle_map[next_position[0]][next_position[1]] == '#':
                current_direction = prev_direction
                break
            current_position = next_position
        continue
    if instruction == 'R':
        if current_direction == (0, 1):
            current_direction = (1, 0)
            continue
        if current_direction == (1, 0):
            current_direction = (0, -1)
            continue
        if current_direction == (0, -1):
            current_direction = (-1, 0)
            continue
        if current_direction == (-1, 0):
            current_direction = (0, 1)
            continue

    if instruction == 'L':
        if current_direction == (0, 1):
            current_direction = (-1, 0)
            continue
        if current_direction == (1, 0):
            current_direction = (0, 1)
            continue
        if current_direction == (0, -1):
            current_direction = (1, 0)
            continue
        if current_direction == (-1, 0):
            current_direction = (0, -1)
            continue


final_row = current_position[0] + 1
final_column = current_position[1] + 1
direction_value = {
    (0,1) : 0,
    (1,0) : 1,
    (0,-1) : 2,
    (-1,0) : 3,
    }

print(1000 * final_row + 4 * final_column + direction_value[current_direction])

#part 2

borderLink = {}


if fileName == "test_input":
    current_direction = (0, 1)
    current_position = (0, 8)

    linkBorder((0, 12), (1, 0), (11, 15), (-1, 0), 4, (0, 1), (0, -1), borderLink)
    linkBorder((11, 16), (-1, 0), (0, 11), (1, 0), 4, (0, 1), (0, -1), borderLink)

    linkBorder((4, 12), (1, 0), (8, 15), (0, -1), 4, (0, 1), (1, 0), borderLink)
    linkBorder((7, 15), (0, -1), (4, 11), (1, 0), 4, (-1, 0), (0, -1), borderLink)

    linkBorder((-1, 8), (0, 1), (4, 3), (0, -1), 4, (-1, 0), (1, 0), borderLink)
    linkBorder((3, 3), (0, -1), (0, 8), (0, 1), 4, (-1, 0), (1, 0), borderLink)

    linkBorder((3, 7), (-1, 0), (4, 7), (0, -1), 4, (0, -1), (1, 0), borderLink)
    linkBorder((3, 7), (0, -1), (3, 8), (-1, 0), 4, (-1, 0), (0, 1), borderLink)

    linkBorder((8, 7), (1, 0), (7, 7), (0, -1), 4, (0, -1), (-1, 0), borderLink)
    linkBorder((8, 7), (0, -1), (8, 8), (1, 0), 4, (1, 0), (0, 1), borderLink)

    linkBorder((12, 8), (0, 1), (7, 3), (0, -1), 4, (1, 0), (-1, 0), borderLink)
    linkBorder((8, 3), (0, -1), (11, 8), (0, 1), 4, (1, 0), (-1, 0), borderLink)

    linkBorder((12, 15), (0, -1), (4, 0), (1, 0), 4, (1, 0), (0, 1), borderLink)
    linkBorder((4, -1), (1, 0), (11, 15), (0, -1), 4, (0, -1), (-1, 0), borderLink)

if fileName == "input":
    current_direction = (0, 1)
    current_position = (0, 50)

    linkBorder((49, 150), (-1, 0), (100, 99), (1, 0), 50, (0, 1), (0, -1), borderLink)
    linkBorder((100, 100), (1, 0), (49, 149), (-1, 0), 50, (0, 1), (0, -1), borderLink)

    linkBorder((50, 100), (1, 0), (49, 100), (0, 1), 50, (0, 1), (-1, 0), borderLink)
    linkBorder((50, 100), (0, 1), (50, 99), (1, 0), 50, (1, 0), (0, -1), borderLink)

    linkBorder((150, 50), (1, 0), (149, 50), (0, 1), 50, (0, 1), (-1, 0), borderLink)
    linkBorder((150, 50), (0, 1), (150, 49), (1, 0), 50, (1, 0), (0, -1), borderLink)

    linkBorder((99, 49), (0, -1), (99, 50), (-1, 0), 50, (-1, 0), (0, 1), borderLink)
    linkBorder((99, 49), (-1, 0), (100, 49), (0, -1), 50, (0, -1), (1, 0), borderLink)

    linkBorder((49, 49), (-1, 0), (100, 0), (1, 0), 50, (0, -1), (0, 1), borderLink)
    linkBorder((100, -1), (1, 0), (49, 50), (-1, 0), 50, (0, -1), (0, 1), borderLink)

    linkBorder((-1, 50), (0, 1), (150, 0), (1, 0), 50, (-1, 0), (0, 1), borderLink)
    linkBorder((150, -1), (1, 0), (0, 50), (0, 1), 50, (0, -1), (1, 0), borderLink)

    linkBorder((-1, 100), (0, 1), (199, 0), (0, 1), 50, (-1, 0), (-1, 0), borderLink)
    linkBorder((200, 0), (0, 1), (0, 100), (0, 1), 50, (1, 0), (1, 0), borderLink)

for instruction in instruction_list:
    if type(instruction) is type(int(1)):
        for i in range(instruction):
            next_position = tuple(map(operator.add, current_position, current_direction))
            prev_direction = current_direction
            if (next_position, current_direction) in borderLink.keys():
                next_position, current_direction = borderLink[(next_position, current_direction)]
            if puzzle_map[next_position[0]][next_position[1]] == '#':
                current_direction = prev_direction
                break
            current_position = next_position
        continue
    if instruction == 'R':
        if current_direction == (0, 1):
            current_direction = (1, 0)
            continue
        if current_direction == (1, 0):
            current_direction = (0, -1)
            continue
        if current_direction == (0, -1):
            current_direction = (-1, 0)
            continue
        if current_direction == (-1, 0):
            current_direction = (0, 1)
            continue

    if instruction == 'L':
        if current_direction == (0, 1):
            current_direction = (-1, 0)
            continue
        if current_direction == (1, 0):
            current_direction = (0, 1)
            continue
        if current_direction == (0, -1):
            current_direction = (1, 0)
            continue
        if current_direction == (-1, 0):
            current_direction = (0, -1)
            continue


final_row = current_position[0] + 1
final_column = current_position[1] + 1
direction_value = {
    (0,1) : 0,
    (1,0) : 1,
    (0,-1) : 2,
    (-1,0) : 3,
    }

print(1000 * final_row + 4 * final_column + direction_value[current_direction])
