package main

import (
	"bufio"
	"log"
	"os"
	"fmt"
	"strconv"
)

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2{
		log.Println("Program takes exactly one parameter")
		log.Fatal("Usage: day1 <input fileName>")
	}
	file, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	//part 1
	scanner := bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	sum := 0
	firstDigit := rune(0)
	lastDigit := rune(0)
	for scanner.Scan(){
		firstDigit = rune(0)
		lastDigit = rune(0)
		for _, char := range scanner.Text(){
			if char <= '9' && char >= '0'{
				if firstDigit == rune(0){
					firstDigit = char
					lastDigit = char
					continue
				}
				lastDigit = char
			}
		}
		if firstDigit == rune(0) {
			log.Fatalf("Line %s has not valid digit", scanner.Text())
		}
		lineValue, err := strconv.Atoi(fmt.Sprintf("%c%c", firstDigit, lastDigit))
		if err != nil{
			log.Fatal(err)
		}
		sum += lineValue
	}
	log.Printf("Result of part 1: %d", sum)

	//part 2
	numberWordMap := make(map[string]rune)
	numberWordMap["one"] = '1'
	numberWordMap["two"] = '2'
	numberWordMap["three"] = '3'
	numberWordMap["four"] = '4'
	numberWordMap["five"] = '5'
	numberWordMap["six"] = '6'
	numberWordMap["seven"] = '7'
	numberWordMap["eight"] = '8'
	numberWordMap["nine"] = '9'

	file.Close()
	file, err = os.Open(os.Args[1])
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()
	scanner = bufio.NewScanner(file)
	scanner.Split(bufio.ScanLines)
	sum = 0
	firstDigit = rune(0)
	lastDigit = rune(0)
	for scanner.Scan(){
		firstDigit = rune(0)
		lastDigit = rune(0)
		for index, char := range scanner.Text(){
			if char <= '9' && char >= '0'{
				if firstDigit == rune(0){
					firstDigit = char
					lastDigit = char
					continue
				}
				lastDigit = char
				continue
			}
			for word := range numberWordMap{
				if len(scanner.Text()[index:]) < len(word){
					continue
				}
				if scanner.Text()[index:index+len(word)] == word{
					if firstDigit == 0{
						firstDigit = numberWordMap[word]
						lastDigit = numberWordMap[word]
						continue
					}
					lastDigit = numberWordMap[word]
					continue
				}
			}

		}
		if firstDigit == rune(0) {
			log.Fatalf("Line %s has not valid digit", scanner.Text())
		}
		lineValue, err := strconv.Atoi(fmt.Sprintf("%c%c", firstDigit, lastDigit))
		if err != nil{
			log.Fatal(err)
		}
		sum += lineValue
	}
	log.Printf("Result of part 2: %d", sum)
}
