package main

import (
	"bufio"
	"fmt"
	"io"
	"log"
	"os"
	"slices"
)

type vec2d struct{
	l int
	c int
}

func AbsInt (x int) int{
	if x < 0{
		return -x
	}
	return x
}

func measure0(v1 vec2d, v2 vec2d) int{
	return AbsInt(v1.l - v2.l) + AbsInt(v1.c - v2.c)
}

func printChart(c [][]uint8){
	for i:=range(c){
		for j:=range(c[i]){
			fmt.Printf("%c", c[i][j])
		}
		fmt.Printf("\n")
	}
}

func getNthColumn(c [][]uint8, n int)[]uint8{
	res := make([]uint8, 0)
	for _,line := range(c){
		res = append(res, line[n])
	}
	return res
}

func insertNthColumn(c [][]uint8, n int, s []uint8) [][]uint8{
	for i := range(c){
		c[i] = slices.Insert(c[i],n,s[i])
	}
	return c
}

func isSliceFree(s []uint8) bool{
	for _, v := range(s){
		if v != '.'{
			return false
		}
	}
	return true
}

func expandSpace(c [][]uint8) ([][]uint8){
	//expand lines
	insertOffset := 0
	insertAt := make([]int, 0)
	for i, line := range(c){
		if isSliceFree(line){
			insertAt = append(insertAt, i+insertOffset)
			insertOffset++
		}
	}
	for _, v := range(insertAt){
		newLine := make([]uint8, 0)
		for range(c[0]){
			newLine = append(newLine, '.')
		}
		c = slices.Insert(c, v, newLine)
	}
	//expand columns
	insertOffset = 0
	insertAt = make([]int, 0)
	for i := range(c[0]){
		if isSliceFree(getNthColumn(c,i)){
			insertAt = append(insertAt, i+insertOffset)
			insertOffset++
		}
	}
	for _, v := range(insertAt){
		newColumn := make([]uint8, 0)
		for range(c){
			newColumn = append(newColumn, '.')
		}
		c = insertNthColumn(c, v, newColumn)
	}

	return c
}

func totalDistanceIncrease(c [][]uint8, galaxyPositions []vec2d, expansion int) int{
	galaxiesLeft := make(map[int]int)
	galaxiesAbove := make(map[int]int) //index 0 none, index len(c) all
	galaxiesFound := 0
	for i := range(c){
		galaxiesAbove[i] = galaxiesFound
		for j :=range(c[i]){
			if slices.Contains(galaxyPositions, vec2d{i,j}){
				galaxiesFound++
			}
		}
	}
	galaxiesAbove[len(c)] = galaxiesFound
	galaxiesFound = 0
	for j := range(c[0]){
		galaxiesLeft[j] = galaxiesFound
		for i :=range(c){
			if slices.Contains(galaxyPositions, vec2d{i,j}){
				galaxiesFound++
			}
		}
	}
	galaxiesLeft[len(c[0])] = galaxiesFound

	//expand lines
	sum := 0
	for i, line := range(c){
		if isSliceFree(line){
			sum += expansion * galaxiesAbove[i] * (galaxiesAbove[len(c)] - galaxiesAbove[i])
		}
	}

	//expand columns
	for i := range(c[0]){
		if isSliceFree(getNthColumn(c,i)){
			sum += expansion * galaxiesLeft[i] * (galaxiesLeft[len(c[0])] - galaxiesLeft[i])
		}
	}

	return sum
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
	scanner := bufio.NewScanner(file)
	chart := make([][]uint8, 0)
	for scanner.Scan(){
		chart = append(chart, []uint8(scanner.Text()))
	}

	chart = expandSpace(chart)
	galaxyPositions := make([]vec2d, 0)
	for i := range(chart){
		for j := range(chart[i]){
			if chart[i][j] == '#'{
				galaxyPositions = append(galaxyPositions, vec2d{i,j})
			}
		}
	}
	sum := 0
	for i ,g1 := range(galaxyPositions){
		for _ ,g2 := range(galaxyPositions[i:]){
			sum += measure0(g1, g2)
		}
	}
	log.Println("result part 1:", sum)

	// part 2
	file.Seek(io.SeekStart, 0)
	scanner = bufio.NewScanner(file)
	chart = make([][]uint8, 0)
	for scanner.Scan(){
		chart = append(chart, []uint8(scanner.Text()))
	}
	galaxyPositions = make([]vec2d, 0)
	for i := range(chart){
		for j := range(chart[i]){
			if chart[i][j] == '#'{
				galaxyPositions = append(galaxyPositions, vec2d{i,j})
			}
		}
	}
	sum = 0
	for i ,g1 := range(galaxyPositions){
		for _ ,g2 := range(galaxyPositions[i:]){
			sum += measure0(g1, g2)
		}
	}
	sum += totalDistanceIncrease(chart, galaxyPositions, 1000000-1)

	log.Println("result part 2:", sum)
	return
}
