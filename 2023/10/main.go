package main

import (
	"bufio"
	"log"
	"os"
)

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
	chart := make([][]uint8, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan(){
		chart = append(chart, []uint8(scanner.Text()))
		log.Println([]uint8(scanner.Text()))
	}
	// discover starting exits
	sPosition := [2]int{0, 0}
	bLabel:
	for i := range(chart){
		for j := range(chart[i]){
			if chart[i][j] == 'S'{
				sPosition = [2]int{i, j}
				break bLabel
			}
		}
	}
	validExits := make([]uint8, 0)


	return
}
