fileName = "adventofcode.com_2022_day_4_input.txt"
with open(file=fileName, mode="r") as f:
    lines = f.readlines()
count = 0
for line in lines:
    elf0, elf1 = line.split(',')
    elf0 = elf0.split('-')
    elf1 = elf1[:-1].split('-')
    elf0 = list(map(int, elf0))
    elf1 = list(map(int, elf1))
    if (
     (elf0[0] >= elf1[0] and elf0[1] <= elf1[1]) or
     (elf1[0] >= elf0[0] and elf1[1] <= elf0[1])):
        count += 1
print(count)
count = 0
for line in lines:
    elf0, elf1 = line.split(',')
    elf0 = elf0.split('-')
    elf1 = elf1[:-1].split('-')
    elf0 = list(map(int, elf0))
    elf1 = list(map(int, elf1))
    if (
     (elf1[0] <= elf0[0] <= elf1[1]) or
     (elf1[0] <= elf0[1] <= elf1[1]) or
     (elf0[0] <= elf1[0] <= elf0[1]) or
     (elf0[0] <= elf1[1] <= elf0[1])):
        count += 1
print(count)
