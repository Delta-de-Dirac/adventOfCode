fileName = "adventofcode.com_2022_day_11_input.txt"
# fileName = "test_input.txt"
class Monkey:
	def __init__(self, monkeyStartingLine, monkeyOperationLine, monkeyTestLine, monkeyTrueLine, monkeyFalseLine):
		self.items = list(map(lambda x: int(x.strip()),monkeyStartingLine.split(":")[1].split(",")))
		self.operation = monkeyOperationLine.split("=")[1][:-1]
		self.test = int(monkeyTestLine.split("by")[1].strip())
		self.true = int(monkeyTrueLine.split(" ")[-1][:-1])
		self.false = int(monkeyFalseLine.split(" ")[-1][:-1])
		self.inspected = 0
		
	def applyOperation(self, item):
		return (lambda old: eval(self.operation))(item)
		
	def applyTest(self, item):
		return (item%self.test) == 0

monkeys = []
with open(file=fileName, mode="r") as f:
	while True:
		try:
			monkeyNumberLine = next(f)
			monkeyStartingLine = next(f)
			monkeyOperationLine = next(f)
			monkeyTestLine = next(f)
			monkeyTrueLine = next(f)
			monkeyFalseLine = next(f)
			monkey = Monkey(monkeyStartingLine, monkeyOperationLine, monkeyTestLine, monkeyTrueLine, monkeyFalseLine)
			monkeys.append(monkey)
			monkeyBlankLine = next(f)		
		except StopIteration:
			break

for i in range(20):
	for monkey in monkeys:
		while len(monkey.items) > 0:
			item = monkey.items.pop(0)
			item = monkey.applyOperation(item)
			monkey.inspected += 1
			item = item//3
			if monkey.applyTest(item):
				monkeys[monkey.true].items.append(item)
			else:
				monkeys[monkey.false].items.append(item)

mostInspection = [0,0]
for monkey in monkeys:
	if mostInspection[0] < monkey.inspected:
		if mostInspection[1] < mostInspection[0]:
			mostInspection[1] = mostInspection[0]
		mostInspection[0] = monkey.inspected
		continue
	if mostInspection[1] < monkey.inspected:
		mostInspection[1] = monkey.inspected
print(mostInspection[0]*mostInspection[1])
	
monkeys = []
with open(file=fileName, mode="r") as f:
	while True:
		try:
			monkeyNumberLine = next(f)
			monkeyStartingLine = next(f)
			monkeyOperationLine = next(f)
			monkeyTestLine = next(f)
			monkeyTrueLine = next(f)
			monkeyFalseLine = next(f)
			monkey = Monkey(monkeyStartingLine, monkeyOperationLine, monkeyTestLine, monkeyTrueLine, monkeyFalseLine)
			monkeys.append(monkey)
			monkeyBlankLine = next(f)		
		except StopIteration:
			break

moduliOfInterest = {monkey.test for monkey in monkeys}


for monkey in monkeys:
	new_items = []
	for item in monkey.items:
		new_item = dict()
		for modulus in moduliOfInterest:
			new_item[modulus] = item%modulus
		new_items.append(new_item)
	monkey.items = new_items
		

for i in range(10000):
	for monkey in monkeys:
		while len(monkey.items) > 0:
			item = monkey.items.pop(0)
			for modulus in item:
				item[modulus] = monkey.applyOperation(item[modulus])%modulus
			monkey.inspected += 1
			if monkey.applyTest(item[monkey.test]):
				monkeys[monkey.true].items.append(item)
			else:
				monkeys[monkey.false].items.append(item)

mostInspection = [0,0]
for monkey in monkeys:
	if mostInspection[0] < monkey.inspected:
		if mostInspection[1] < mostInspection[0]:
			mostInspection[1] = mostInspection[0]
		mostInspection[0] = monkey.inspected
		continue
	if mostInspection[1] < monkey.inspected:
		mostInspection[1] = monkey.inspected
print(mostInspection[0]*mostInspection[1])
