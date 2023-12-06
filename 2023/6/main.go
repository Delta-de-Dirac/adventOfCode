package main

import (
	"log"
	"os"
	"bufio"
	"strings"
	"math"
	"strconv"
)

func getAmountOfWaysToWin(mt, md int) int{
	if mt*mt <= 4*md{
		return 0
	}
	lb := float64(mt)/2 - math.Sqrt(float64(mt*mt-4*md))/2.0
	ub := float64(mt)/2 + math.Sqrt(float64(mt*mt-4*md))/2.0
	return int(math.Ceil(ub)) - int(math.Floor(lb)) - 1
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: day6 <input fileName>")
	}

	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)
	scanner.Scan()
	mts := make([]int, 0)
	mds := make([]int, 0)
	for _, field := range strings.Fields(scanner.Text())[1:]{
		num, err := strconv.Atoi(field)
		if err != nil{
			log.Fatal(err)
		}
		mts = append(mts, num)
	}
	scanner.Scan()
	for _, field := range strings.Fields(scanner.Text())[1:]{
		num, err := strconv.Atoi(field)
		if err != nil{
			log.Fatal(err)
		}
		mds = append(mds, num)
	}

	// part 1

	product := 1
	for i := range mts{
		product *= getAmountOfWaysToWin(mts[i],mds[i])
	}
	log.Println("Result part 1:", product)

	// part 2
	return
}
