package main

import (
	"bufio"
	"log"
	"os"
	"strings"
)

type Node struct{
	l string
	r string
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: day7 <input fileName>")
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
	for scanner.Scan(){
		fields := strings.FieldsFunc(scanner.Text(),
				   func(r rune) bool {
					   return strings.ContainsRune(" ,()=", r)})
		nodeMap[fields[0]] = Node{fields[1], fields[2]}
	}
	//part 1
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
