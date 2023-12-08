package main

import (
	"bufio"
	"log"
	"os"
	"strings"
	_ "slices"
)



type Node struct{
	l string
	r string
}

type nodeState struct{
	node string
	inst int
}

func gcd(a, b int) int{
	for b != 0{
		t:= b
		b = a%b
		a=t
	}
	return a
}

func lcm(a, b int) int{
	return (a/gcd(a,b))*b
}

func getOrbit(node string, mapNodes map[string]Node, instructions []byte) (stepStable, orbitalPeriod int, reachZ[]int){
	step := 0
	currentNodeState := nodeState{node, 0}
	stepAtNodeState := make(map[nodeState]int)
	stepFound := -1
	ok := false
	for {
		stepFound, ok = stepAtNodeState[currentNodeState]
		if ok{
			stepStable = step
			break
		}
		stepAtNodeState[currentNodeState] = step
		switch instructions[currentNodeState.inst]{
			case 'R':
				currentNodeState.node = mapNodes[currentNodeState.node].r
			case 'L':
				currentNodeState.node = mapNodes[currentNodeState.node].l
			default:
				log.Fatal("error")
		}
		step++
		currentNodeState.inst++
		if currentNodeState.inst >= len(instructions){
			currentNodeState.inst = 0
		}
	}
	orbitalPeriod = stepStable-stepFound
	reachZ = make([]int, 0)
	for i:=0;i<orbitalPeriod;i++{
		if currentNodeState.node[2] == 'Z'{
			reachZ = append(reachZ, i)
		}
		switch instructions[currentNodeState.inst]{
			case 'R':
				currentNodeState.node = mapNodes[currentNodeState.node].r
			case 'L':
				currentNodeState.node = mapNodes[currentNodeState.node].l
			default:
				log.Fatal("error")
		}
		step++
		currentNodeState.inst++
		if currentNodeState.inst >= len(instructions){
			currentNodeState.inst = 0
		}
	}

	return stepStable, orbitalPeriod, reachZ
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatalf("Usage: %s <input fileName>", os.Args[0])
	}

	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	nodeMap := make(map[string]Node)
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	instructions := []byte(scanner.Text())
	scanner.Scan()//empty line
	hasAAA := false
	hasZZZ := false
	for scanner.Scan(){
		fields := strings.FieldsFunc(scanner.Text(),
				   func(r rune) bool {
					   return strings.ContainsRune(" ,()=", r)})
		nodeMap[fields[0]] = Node{fields[1], fields[2]}
		if fields[0] == "AAA"{
			hasAAA = true
		}
		if fields[0] == "ZZZ"{
			hasZZZ = true
		}
	}
	//part 1
	if hasAAA && hasZZZ{
		step := 0
		currentNode := "AAA"
		instruction := 0
		for {
			if instructions[instruction] == 'L'{
				currentNode = nodeMap[currentNode].l
				step++
				if currentNode == "ZZZ"{
					break
				}
				instruction++
				if instruction >= len(instructions){
					instruction = 0
				}
				continue
			}
			if instructions[instruction] == 'R'{
				currentNode = nodeMap[currentNode].r
				step++
				if currentNode == "ZZZ"{
					break
				}
				instruction++
				if instruction >= len(instructions){
					instruction = 0
				}
				continue
			}
			log.Fatal("err")
		}
		log.Println("Result part 1:", step)
	}

	//part 2
	myNodes := make([]string, 0)
	f := make([]int, 0)
	valueToFindLcm := make([]int, 0)
	for k := range nodeMap{
		if k[2] == 'A'{
			myNodes = append(myNodes, k)
			e, op, rz := getOrbit(k, nodeMap, instructions)//only 1 rz... why??
			f = append(f, (e+rz[0])%op)
			valueToFindLcm = append(valueToFindLcm, op)
		}
	}
	log.Println(f) //all 0... why??

	// knowing we have only 1 rz and (e+rz[0])%op == 0 for every node path
	// we simply need find lcm of ops
	myLcm := 1
	for _, op:= range valueToFindLcm{
		myLcm = lcm(myLcm, op)
	}
	log.Println("Result part 2:", myLcm)
}
