package main

import (
	"bufio"
	"strconv"
	"log"
	"os"
	"strings"
)

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: day2 <input fileName>")
	}

	// part 1
	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	gameID := 1
	sum := 0

	maxRed := 12
	maxGreen := 13
	maxBlue := 14

	for scanner.Scan(){
		gamePossible := true
		gameContent := strings.Split(scanner.Text(), ":")
		if len(gameContent) != 2{
			log.Fatalf("Unexpected line format: %s", scanner.Text())
		}
		gameSets := strings.Split(gameContent[1], ";")
		for _, set := range gameSets{
			for _, cube := range strings.Split(set, ","){
				cube = strings.Trim(cube, " ")
				cubeColor := strings.Split(cube, " ")[1]
				cubeAmount, err := strconv.Atoi(strings.Split(cube, " ")[0])
				if err != nil{
					log.Fatal(err)
				}
				switch cubeColor{
					case "red":
						if cubeAmount > maxRed{
							gamePossible = false
						}
					case "green":
						if cubeAmount > maxGreen{
							gamePossible = false
						}
					case "blue":
						if cubeAmount > maxBlue{
							gamePossible = false
						}
					default:
						log.Fatalf("Unexpected line: %s", scanner.Text())
				}
				if !gamePossible{
					break
				}
			}
			if !gamePossible{
				break
			}
		}
		if !gamePossible{
			gameID++
			continue
		}
		sum += gameID
		gameID++
	}
	log.Printf("Result for part 1: %d", sum)

	// part 2
	file.Close()
	file, err = os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	scanner = bufio.NewScanner(file)

	sum = 0
	for scanner.Scan(){
		maxRedFound := 0
		maxGreenFound := 0
		maxBlueFound := 0
		gameContent := strings.Split(scanner.Text(), ":")
		if len(gameContent) != 2{
			log.Fatalf("Unexpected line format: %s", scanner.Text())
		}
		gameSets := strings.Split(gameContent[1], ";")
		for _, set := range gameSets{
			for _, cube := range strings.Split(set, ","){
				cube = strings.Trim(cube, " ")
				cubeColor := strings.Split(cube, " ")[1]
				cubeAmount, err := strconv.Atoi(strings.Split(cube, " ")[0])
				if err != nil{
					log.Fatal(err)
				}
				switch cubeColor{
					case "red":
						if cubeAmount > maxRedFound{
							maxRedFound = cubeAmount
						}
					case "green":
						if cubeAmount > maxGreenFound{
							maxGreenFound = cubeAmount
						}
					case "blue":
						if cubeAmount > maxBlueFound{
							maxBlueFound = cubeAmount
						}
					default:
						log.Fatalf("Unexpected line: %s", scanner.Text())
				}
			}
		}
		sum += maxRedFound * maxGreenFound * maxBlueFound
	}
	log.Printf("Result for part 2: %d", sum)
}
