fileName = "input"

monkeyDict = {}


def evalMonkey(monkey, monkeyDict):
    if type(monkeyDict[monkey]) is type(int(1)):
        return monkeyDict[monkey]
    if type(monkeyDict[monkey]) is type(dict()):
        monkeyDict[monkey] = int(eval(str(evalMonkey(monkeyDict[monkey]['operand1'], monkeyDict)) + monkeyDict[monkey]['operator'] + str(evalMonkey(monkeyDict[monkey]['operand2'], monkeyDict))))
        return monkeyDict[monkey]


with open(file=fileName, mode="r") as f:
    for line in f:
        content = line.split(' ')
        if len(content) == 2:
            monkeyDict[content[0][:4]] = int(content[1].strip())
            continue
        if len(content) == 4:
            monkeyDict[content[0][:4]] = {
                'operand1': content[1][:4],
                'operand2': content[3][:4],
                'operator': content[2][0]
            }
            continue
        print("error")

print(evalMonkey('root', monkeyDict))

# part 2

monkeyDict = {}

with open(file=fileName, mode="r") as f:
    for line in f:
        content = line.split(' ')
        if len(content) == 2:
            monkeyDict[content[0][:4]] = int(content[1].strip())
            continue
        if len(content) == 4:
            monkeyDict[content[0][:4]] = {
                'operand1': content[1][:4],
                'operand2': content[3][:4],
                'operator': content[2][0]
            }
            continue
        print("error")

class Node():
    def __init__(self, left=None, right=None, value=None):
        self.left = left
        self.right = right
        self.value = value

    def has_x(self):
        if self.value == 'x':
            return True
        if type(self.value) is type(int(1)):
            return False
        return self.left.has_x() or self.right.has_x()

    def eval_expression(self):
        if self.has_x():
            print("can't eval if has x")
            return None
        if type(self.value) is type(int(1)):
            return self.value
        if self.value == '+':
            return self.left.eval_expression() + self.right.eval_expression()
        if self.value == '-':
            return self.left.eval_expression() - self.right.eval_expression()
        if self.value == '*':
            return self.left.eval_expression() * self.right.eval_expression()
        if self.value == '/':
            return self.left.eval_expression() / self.right.eval_expression()
        print('error')
        return None

def build_expression(monkey, monkeyDict):
    if type(monkeyDict[monkey]) is type(int(1)) or type(monkeyDict[monkey]) is type(str('')):
        return Node(value=monkeyDict[monkey])
    if type(monkeyDict[monkey]) is type(dict()):
        return Node(
            left=build_expression(monkeyDict[monkey]['operand1'], monkeyDict),
            right=build_expression(monkeyDict[monkey]['operand2'], monkeyDict),
            value=monkeyDict[monkey]['operator']
        )

def solve(expression_left, expression_right):
    if expression_left.value == 'x':
        return expression_right.eval_expression()
    if not expression_left.has_x():
        return solve(expression_right, expression_left)
    if expression_left.left.has_x():
        if expression_left.value == '+':
            return solve(
                expression_left.left,
                Node(
                    left=expression_right,
                    right=expression_left.right,
                    value='-'
                    )
                )
        if expression_left.value == '-':
            return solve(
                expression_left.left,
                Node(
                    left=expression_right,
                    right=expression_left.right,
                    value='+'
                    )
                )
        if expression_left.value == '*':
            return solve(
                expression_left.left,
                Node(
                    left=expression_right,
                    right=expression_left.right,
                    value='/'
                    )
                )
        if expression_left.value == '/':
            return solve(
                expression_left.left,
                Node(
                    left=expression_right,
                    right=expression_left.right,
                    value='*'
                    )
                )
    if expression_left.value == '+':
        return solve(
            expression_left.right,
            Node(
                left=expression_right,
                right=expression_left.left,
                value='-'
                )
            )
    if expression_left.value == '-':
        return solve(
            expression_left.right,
            Node(
                left=expression_left.left,
                right=expression_right,
                value='-'
                )
            )
    if expression_left.value == '*':
        return solve(
            expression_left.right,
            Node(
                left=expression_right,
                right=expression_left.left,
                value='/'
                )
            )
    if expression_left.value == '/':
        return solve(
            expression_left.right,
            Node(
                left=expression_left.left,
                right=expression_right,
                value='/'
                )
            )
    print('error')
    return None

monkeyDict['humn'] = 'x'

expression_left = build_expression('root', monkeyDict).left

expression_right = build_expression('root', monkeyDict).right

print(solve(expression_left, expression_right))
