import operator
fileName = "test_input"


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


def linkBorder(origin1, direction1, origin2, direction2, length, borderLink):
    pos1 = origin1
    pos2 = origin2
    for i in range(length):
        borderLink[pos1] = pos2
        pos1 = tuple(map(operator.add, pos1, direction1))
        pos2 = tuple(map(operator.add, pos2, direction2))


puzzle_map = []


borderLink = {}


with open(file=fileName, mode="r") as f:
    for line in f:
        line_content = line.strip('\r\n')
        if line_content == '':
            break
        puzzle_map.append(line_content)
    instructions = f.readline()

instruction_list = decode_instructions(instructions)

if fileName == "test_input":
    current_position = (0, 8)

    linkBorder((0, 12), (1, 0), (0, 8), (1, 0), 4, borderLink)
    linkBorder((0, 7), (1, 0), (0, 11), (1, 0), 4, borderLink)

    linkBorder((4, 12), (1, 0), (4, 0), (1, 0), 4, borderLink)
    linkBorder((4, -1), (1, 0), (4, 11), (1, 0), 4, borderLink)

    linkBorder((8, 16), (1, 0), (8, 8), (1, 0), 4, borderLink)
    linkBorder((8, 7), (1, 0), (8, 15), (1, 0), 4, borderLink)


    linkBorder((8, 0), (0, 1), (4, 0), (0, 1), 4, borderLink)
    linkBorder((3, 0), (0, 1), (7, 0), (0, 1), 4, borderLink)

    linkBorder((8, 4), (0, 1), (4, 4), (0, 1), 4, borderLink)
    linkBorder((3, 4), (0, 1), (7, 4), (0, 1), 4, borderLink)

    linkBorder((12, 8), (0, 1), (0, 8), (0, 1), 4, borderLink)
    linkBorder((-1, 8), (0, 1), (11, 8), (0, 1), 4, borderLink)

    linkBorder((12, 12), (0, 1), (8, 12), (0, 1), 4, borderLink)
    linkBorder((7, 12), (0, 1), (11, 12), (0, 1), 4, borderLink)

if fileName == "input":
    current_position = (0, 50)

    linkBorder((0, 12), (1, 0), (0, 8), (1, 0), 4, borderLink)
    linkBorder((0, 7), (1, 0), (0, 11), (1, 0), 4, borderLink)

    linkBorder((4, 12), (1, 0), (4, 0), (1, 0), 4, borderLink)
    linkBorder((4, -1), (1, 0), (4, 11), (1, 0), 4, borderLink)

    linkBorder((8, 16), (1, 0), (8, 8), (1, 0), 4, borderLink)
    linkBorder((8, 7), (1, 0), (8, 15), (1, 0), 4, borderLink)


    linkBorder((8, 0), (0, 1), (4, 0), (0, 1), 4, borderLink)
    linkBorder((3, 0), (0, 1), (7, 0), (0, 1), 4, borderLink)

    linkBorder((8, 4), (0, 1), (4, 4), (0, 1), 4, borderLink)
    linkBorder((3, 4), (0, 1), (7, 4), (0, 1), 4, borderLink)

    linkBorder((12, 8), (0, 1), (0, 8), (0, 1), 4, borderLink)
    linkBorder((-1, 8), (0, 1), (11, 8), (0, 1), 4, borderLink)

    linkBorder((12, 12), (0, 1), (8, 12), (0, 1), 4, borderLink)
    linkBorder((7, 12), (0, 1), (11, 12), (0, 1), 4, borderLink)

current_direction = (0, 1)

for instruction in instruction_list:
    if type(instruction) is type(int(1)):
        for i in range(instruction):
            next_position = tuple(map(operator.add, current_position, current_direction))
            if next_position in borderLink.keys():
                next_position = borderLink[next_position]
            if puzzle_map[next_position[0]][next_position[1]] == '#':
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

