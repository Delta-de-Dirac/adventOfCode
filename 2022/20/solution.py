fileName = "test_input"

class Node:
    def __init__(self,data,prev=None):
        self.data = data
        self.prev = prev
        self.next = None
    
    def swapPrev(self):
        self.prev.next = self.next
        self.next = self.prev
        self.prev = self.prev.prev
        self.prev.next.prev = self
        self.prev.next = self
        self.next.next.prev = self.next
        
    def swapNext(self):
        self.prev.next = self.next
        self.next.prev = self.prev
        self.next.next.prev = self
        self.next = self.next.next
        self.prev.next.next = self
        self.prev = self.prev.next
        
        
    def printList(self):
        print(self.data, end=' ')
        currentNode = self.next
        while currentNode != self:
            print(currentNode.data, end=' ')
            currentNode = currentNode.next
        print()

nodeStack = []

zeroNode = None

with open(file=fileName, mode="r") as f:
    prevNode = None
    for index, line in enumerate(f):
        data = int(line.strip())
        newNode = Node(data, prevNode)
        if data == 0:
            zeroNode = newNode
        if prevNode != None:
            prevNode.next = newNode
        nodeStack.append(newNode)
        prevNode = newNode
nodeStack[0].prev = nodeStack[-1]
nodeStack[-1].next = nodeStack[0]

length = len(nodeStack)

for node in nodeStack:
    if node.data >= 0:
        for i in range(node.data%(length-1)):
            node.swapNext()
        continue
    for i in range((-node.data)%(length-1)):
        node.swapPrev()

sum = 0

currentNode = zeroNode

for i in range(3):
    for j in range(1000%length):
        currentNode = currentNode.next
    sum += currentNode.data
    
print(sum)

#part 2


dec_key = 811589153

nodeStack = []

zeroNode = None

with open(file=fileName, mode="r") as f:
    prevNode = None
    for index, line in enumerate(f):
        data = int(line.strip())
        newNode = Node(data, prevNode)
        if data == 0:
            zeroNode = newNode
        if prevNode != None:
            prevNode.next = newNode
        nodeStack.append(newNode)
        prevNode = newNode
nodeStack[0].prev = nodeStack[-1]
nodeStack[-1].next = nodeStack[0]

length = len(nodeStack)

for i in range(10):
    for node in nodeStack:
        if node.data >= 0:
            for i in range(((node.data%(length-1))*dec_key)%(length-1)):
                node.swapNext()
            continue
        for i in range((((-node.data)%(length-1))*dec_key)%(length-1)):
            node.swapPrev()

sum = 0

currentNode = zeroNode

for i in range(3):
    for j in range(1000%length):
        currentNode = currentNode.next
    sum += currentNode.data*dec_key
    
print(sum)

