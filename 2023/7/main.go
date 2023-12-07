package main

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func getCardVal(c byte) int{
	switch c{
		case 'T':
			return 8
		case 'J':
			return 9
		case 'Q':
			return 10
		case 'K':
			return 11
		case 'A':
			return 12
		default:
			return int(c - '2')
	}
}

func getScore(hand string) int{
	count := make(map[byte]int)
	for i := range hand{
		_, ok := count[hand[i]]
		if ok{
			count[hand[i]] = count[hand[i]] + 1
			continue
		}
		count[hand[i]] = 1
	}
	highestCount := -1
	secondHighestCount := -1
	for _, v := range count{
		if v >= highestCount{
			secondHighestCount = highestCount
			highestCount = v
			continue
		}
		if v >= secondHighestCount{
			secondHighestCount = v
		}
	}
	if highestCount == 5{
		return 6
	}
	if highestCount == 4{
		return 5
	}
	if highestCount == 3{
		if secondHighestCount == 2{
			return 4
		}
		return 3
	}
	if highestCount == 2{
		if secondHighestCount == 2{
			return 2
		}
		return 1
	}
	return 0
}

func getCardVal2(c byte) int{
	switch c{
		case 'T':
			return 9
		case 'J':
			return 0
		case 'Q':
			return 10
		case 'K':
			return 11
		case 'A':
			return 12
		default:
			return int(c - '2') + 1
	}
}

func getScore2(hand string) int{
	count := make(map[byte]int)
	if hand == "JJJJJ"{
		return 6
	}
	for i := range hand{
		_, ok := count[hand[i]]
		if ok{
			count[hand[i]] = count[hand[i]] + 1
			continue
		}
		count[hand[i]] = 1
	}
	highestCount := -1
	secondHighestCount := -1
	anyJ := false
	for k, v := range count{
		if k == 'J'{
			anyJ = true
		}
		if v >= highestCount{
			secondHighestCount = highestCount
			highestCount = v
			continue
		}
		if v >= secondHighestCount{
			secondHighestCount = v
		}
	}
	if anyJ{
		highestScore := 0
		for k := range count{
			if k == 'J'{
				continue
			}
			testScore := getScore(strings.ReplaceAll(hand, "J", string(k)))
			if testScore > highestScore{
				highestScore = testScore
			}
		}
		return highestScore
	}

	if highestCount == 5{
		return 6
	}
	if highestCount == 4{
		return 5
	}
	if highestCount == 3{
		if secondHighestCount == 2{
			return 4
		}
		return 3
	}
	if highestCount == 2{
		if secondHighestCount == 2{
			return 2
		}
		return 1
	}
	return 0
}

func isLower(handA, handB string) bool{
	if getScore(handA) < getScore(handB){
		return true
	}
	if getScore(handA) > getScore(handB){
		return false
	}
	for i:=range handA{
		if getCardVal(handA[i]) < getCardVal(handB[i]){
			return true
		}
		if getCardVal(handA[i]) > getCardVal(handB[i]){
			return false
		}
	}
	return true
}

func isLower2(handA, handB string) bool{
	if getScore2(handA) < getScore2(handB){
		return true
	}
	if getScore2(handA) > getScore2(handB){
		return false
	}
	for i:=range handA{
		if getCardVal2(handA[i]) < getCardVal2(handB[i]){
			return true
		}
		if getCardVal2(handA[i]) > getCardVal2(handB[i]){
			return false
		}
	}
	return true
}

func getOrder(hands []string) []int{
	order:=make([]int, 0)
	for i, hand:=range hands{
		inserted := false
		for j:=range order{
			if isLower(hand, hands[order[j]]){
				order = slices.Insert(order, j, i)
				inserted = true
				break
			}
		}
		if inserted{
			continue
		}
		order = append(order, i)
	}

	return order
}

func getOrder2(hands []string) []int{
	order:=make([]int, 0)
	for i, hand:=range hands{
		inserted := false
		for j:=range order{
			if isLower2(hand, hands[order[j]]){
				order = slices.Insert(order, j, i)
				inserted = true
				break
			}
		}
		if inserted{
			continue
		}
		order = append(order, i)
	}

	return order
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
	scanner := bufio.NewScanner(file)
	hands := make([]string,0)
	bids := make([]int,0)
	for scanner.Scan(){
		fields := strings.Fields(scanner.Text())
		hands = append(hands, fields[0])
		num, err := strconv.Atoi(fields[1])
		if err!= nil{
			log.Fatal(err)
		}
		bids = append(bids, num)
	}

	// part 1
	order := getOrder(hands)
	totalWinnings := 0
	for i := range order{
		totalWinnings += (i+1)*bids[order[i]]
	}
	log.Println("Result part 1:", totalWinnings)
	// part 2
	order = getOrder2(hands)
	totalWinnings = 0
	for i := range order{
		totalWinnings += (i+1)*bids[order[i]]
	}
	log.Println("Result part 2:", totalWinnings)
}
