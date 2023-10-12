fileName = "test_input"

with open(file=fileName, mode="r") as f:
    for line in f:
        print(line.strip('\r\n'))
