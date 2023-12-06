package main

import (
	"bufio"
	"strings"
	"log"
	"os"
	"strconv"
)

type game struct{
	winningNumbers []int
	myNumbers []int
	copies int
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: day4 <input fileName>")
	}

	// part 1
	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	games := make([]game, 0)

	for scanner.Scan(){
		lineGame := game{
			winningNumbers: make([]int, 0),
			myNumbers: make([]int, 0),
			copies: 1,
		}
		lineFields := strings.Fields(scanner.Text())
		readingWinningNumbers := false
		readingMyNumbers := false
		for _, field := range lineFields{
			if strings.Contains(field, ":"){
				readingWinningNumbers = true
				continue
			}
			if strings.Contains(field, "|"){
				readingMyNumbers = true
				readingWinningNumbers = false
				continue
			}
			if readingMyNumbers {
				num, err := strconv.Atoi(field)
				if err != nil{
					log.Fatal(err)
				}
				lineGame.myNumbers = append(lineGame.myNumbers, num)
				continue
			}
			if readingWinningNumbers{
				num, err := strconv.Atoi(field)
				if err != nil{
					log.Fatal(err)
				}
				lineGame.winningNumbers = append(lineGame.winningNumbers, num)
				continue
			}
		}
		games = append(games, lineGame)
	}

	// part 1

	totalPoints := 0
	for _, game := range games{
		points := 0
		for _, num := range game.myNumbers{
			for _, winNum := range game.winningNumbers{
				if num == winNum{
					if points == 0{
						points = 1
						break
					}
					points *= 2
					break
				}
			}
		}
		totalPoints += points
	}
	log.Println("Part 1 result:", totalPoints)

	// part 2
	totalCopies := 0
	for gameNum, game := range games{
		matches := 0
		for _, num := range game.myNumbers{
			for _, winNum := range game.winningNumbers{
				if num == winNum{
					matches++
					break
				}
			}
		}
		for i:=1;i<=matches;i++{
			if gameNum+i >= len(games){
				break
			}
			games[gameNum+i].copies+=game.copies
		}
		totalCopies += game.copies
	}
	log.Println("Part 2 result:", totalCopies)
}
