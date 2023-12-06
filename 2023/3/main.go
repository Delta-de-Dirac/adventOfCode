package main

import (
	"log"
	"os"
	"bufio"
	"strconv"
)

func isSymbol(r byte) bool{
	if r >= '0' && r <= '9'{
		return false
	}
	if r == '.'{
		return false
	}
	return true
}

func getAdjacentPartNumbers(l, c int, input []string) (adjacentNumbers []int, ferror error){
	adjacentNumbers = make([]int, 0)
	if l > 0{
		numbers, begin, end, err := getNumberPositions(input[l-1])
		if err != nil{
			return nil, err
		}
		for i := range numbers{
			if c > 0{
				if c-1 >= begin[i] && c-1 <= end[i]{
					adjacentNumbers = append(adjacentNumbers, numbers[i])
					continue
				}
			}
			if c >= begin[i] && c <= end[i]{
				adjacentNumbers = append(adjacentNumbers, numbers[i])
				continue
			}
			if c < len(input[0])-1{
				if c+1 >= begin[i] && c+1 <= end[i]{
					adjacentNumbers = append(adjacentNumbers, numbers[i])
					continue
				}
			}
		}
	}
	if l < len(input)-1{
		numbers, begin, end, err := getNumberPositions(input[l+1])
		if err != nil{
			return nil, err
		}
		for i := range numbers{
			if c > 0{
				if c-1 >= begin[i] && c-1 <= end[i]{
					adjacentNumbers = append(adjacentNumbers, numbers[i])
					continue
				}
			}
			if c >= begin[i] && c <= end[i]{
				adjacentNumbers = append(adjacentNumbers, numbers[i])
				continue
			}
			if c < len(input[0])-1{
				if c+1 >= begin[i] && c+1 <= end[i]{
					adjacentNumbers = append(adjacentNumbers, numbers[i])
					continue
				}
			}
		}
	}
	numbers, begin, end, err := getNumberPositions(input[l])
	if err != nil{
		return nil, err
	}
	for i := range numbers{
		if c > 0{
			if c-1 >= begin[i] && c-1 <= end[i]{
				adjacentNumbers = append(adjacentNumbers, numbers[i])
				continue
			}
		}
		if c >= begin[i] && c <= end[i]{
			adjacentNumbers = append(adjacentNumbers, numbers[i])
			continue
		}
		if c < len(input[0])-1{
			if c+1 >= begin[i] && c+1 <= end[i]{
				adjacentNumbers = append(adjacentNumbers, numbers[i])
				continue
			}
		}
	}
	return adjacentNumbers, nil
}

func getNumberPositions(s string) (numbers []int, begin []int, end []int, ferror error){
	numbers = make([]int, 0)
	begin = make([]int, 0)
	end = make([]int, 0)

	currentBegin := -1
	currentEnd := -1
	digits := ""
	for i, c := range s{
		if c >= '0' && c <= '9'{
			if currentBegin == -1{
				currentBegin = i
			}
			digits = digits + string(c)
			continue
		}
		if currentBegin != -1{
			currentEnd = i-1
			number, err := strconv.Atoi(digits)
			if err != nil{
				return nil, nil, nil, err
			}
			numbers = append(numbers, number)
			begin = append(begin, currentBegin)
			end = append(end, currentEnd)
			currentBegin = -1
			currentEnd = -1
			digits = ""
		}
	}
	if currentBegin != -1{
		currentEnd = len(s)-1
		number, err := strconv.Atoi(digits)
		if err != nil{
			return nil, nil, nil, err
		}
		numbers = append(numbers, number)
		begin = append(begin, currentBegin)
		end = append(end, currentEnd)
		currentBegin = -1
		currentEnd = -1
		digits = ""
	}
	return numbers, begin, end, nil
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2{
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: main <input fileName>")
	}
	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	input := make([]string, 0)
	for scanner.Scan(){
		input = append(input, scanner.Text())
	}

	//part 1

	sum := 0

	// analyze first line
	line := 0
	numbers, begin, end, err := getNumberPositions(input[line])
	if err != nil{
		log.Fatal(err)
	}
	for i := range numbers{
		if begin[i] > 0{
			if isSymbol(input[line][begin[i]-1]){
				sum += numbers[i]
				continue
			}
			if isSymbol(input[line+1][begin[i]-1]){
				sum += numbers[i]
				continue
			}
		}
		for j:=begin[i];j<=end[i];j++{
			if isSymbol(input[line+1][j]){
				sum += numbers[i]
				continue
			}
		}
		if end[i] < len(input[0])-1{
			if isSymbol(input[line][end[i]+1]){
				sum += numbers[i]
				continue
			}
			if isSymbol(input[line+1][end[i]+1]){
				sum += numbers[i]
				continue
			}
		}
	}

	// analyze last line
	line = len(input)-1
	numbers, begin, end, err = getNumberPositions(input[line])
	if err != nil{
		log.Fatal(err)
	}
	for i := range numbers{
		if begin[i] > 0{
			if isSymbol(input[line][begin[i]-1]){
				sum += numbers[i]
				continue
			}
			if isSymbol(input[line-1][begin[i]-1]){
				sum += numbers[i]
				continue
			}
		}
		for j:=begin[i];j<=end[i];j++{
			if isSymbol(input[line-1][j]){
				sum += numbers[i]
				continue
			}
		}
		if end[i] < len(input[0])-1{
			if isSymbol(input[line][end[i]+1]){
				sum += numbers[i]
				continue
			}
			if isSymbol(input[line-1][end[i]+1]){
				sum += numbers[i]
				continue
			}
		}
	}

	// analyze middle lines
	for line:=1;line<len(input)-1;line++{
		numbers, begin, end, err = getNumberPositions(input[line])
		if err != nil{
			log.Fatal(err)
		}
		for i := range numbers{
			if begin[i] > 0{
				if isSymbol(input[line][begin[i]-1]){
					sum += numbers[i]
					continue
				}
				if isSymbol(input[line+1][begin[i]-1]){
					sum += numbers[i]
					continue
				}
				if isSymbol(input[line-1][begin[i]-1]){
					sum += numbers[i]
					continue
				}
			}
			for j:=begin[i];j<=end[i];j++{
				if isSymbol(input[line-1][j]){
					sum += numbers[i]
					continue
				}
				if isSymbol(input[line+1][j]){
					sum += numbers[i]
					continue
				}
			}
			if end[i] < len(input[0])-1{
				if isSymbol(input[line][end[i]+1]){
					sum += numbers[i]
					continue
				}
				if isSymbol(input[line+1][end[i]+1]){
					sum += numbers[i]
					continue
				}
				if isSymbol(input[line-1][end[i]+1]){
					sum += numbers[i]
					continue
				}
			}
		}
	}

	log.Printf("Result part 1: %d\n", sum)

	//part 2
	sum = 0
	product := 1
	for line := range input{
		for c := range input[line]{
			if input[line][c] == '*'{
				adjacentNumbers, err := getAdjacentPartNumbers(line,c,input)
				if err != nil {
					log.Fatal(err)
				}
				if len(adjacentNumbers) >= 2{
					for _, n := range adjacentNumbers{
						product *= n
					}
					sum += product
					product = 1
				}
			}
		}
	}
	log.Printf("Result part 2: %d\n", sum)
}
