package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
	"slices"
)

func printChart(chart [][]uint8, hpos [][2]int){
	for i := range(chart){
		for j := range(chart[i]){
			if slices.Contains(hpos, [2]int{i,j}){
				fmt.Printf("#")
				continue
			}
			fmt.Printf("%c", chart[i][j])
		}
		fmt.Printf("\n")
	}
	return
}

func findSPiece(pos [2]int, validExits [][2]int) uint8{
	vec1 := [2]int{validExits[0][0]-pos[0], validExits[0][1]-pos[1]}
	vec2 := [2]int{validExits[1][0]-pos[0], validExits[1][1]-pos[1]}
	if vec1 == [2]int{1,0} && vec2 == [2]int{-1,0} ||
		vec1 == [2]int{-1,0} && vec2 == [2]int{1,0} {
		return '|'
	}
	if vec1 == [2]int{0,1} && vec2 == [2]int{0,-1} ||
		vec1 == [2]int{0,-1} && vec2 == [2]int{0,1} {
		return '-'
	}
	if vec1 == [2]int{1,0} && vec2 == [2]int{0,-1} ||
		vec1 == [2]int{0,-1} && vec2 == [2]int{1,0} {
		return '7'
	}
	if vec1 == [2]int{1,0} && vec2 == [2]int{0,1} ||
		vec1 == [2]int{0,1} && vec2 == [2]int{1,0} {
		return 'F'
	}
	if vec1 == [2]int{-1,0} && vec2 == [2]int{0,-1} ||
		vec1 == [2]int{0,-1} && vec2 == [2]int{-1,0} {
		return 'J'
	}
	if vec1 == [2]int{-1,0} && vec2 == [2]int{0,1} ||
		vec1 == [2]int{0,1} && vec2 == [2]int{-1,0} {
		return 'L'
	}
	log.Fatal("Cannot fit piece into sPostion")
	return 0
}

func walk(piece uint8, at [2]int, was[2]int) [2]int{
	var pos [2]int
	switch piece{
		case '|':
			if was == [2]int{at[0]-1,at[1]}{
				pos = [2]int{at[0]+1,at[1]}
			} else{
				pos = [2]int{at[0]-1,at[1]}
			}
		case 'F':
			if was == [2]int{at[0],at[1]+1}{
				pos = [2]int{at[0]+1,at[1]}
			} else{
				pos = [2]int{at[0],at[1]+1}
			}
		case '-':
			if was == [2]int{at[0],at[1]-1}{
				pos = [2]int{at[0],at[1]+1}
			} else{
				pos = [2]int{at[0],at[1]-1}
			}
		case 'L':
			if was == [2]int{at[0],at[1]+1}{
				pos = [2]int{at[0]-1,at[1]}
			} else{
				pos = [2]int{at[0],at[1]+1}
			}
		case 'J':
			if was == [2]int{at[0],at[1]-1}{
				pos = [2]int{at[0]-1,at[1]}
			} else{
				pos = [2]int{at[0],at[1]-1}
			}
		case '7':
			if was == [2]int{at[0],at[1]-1}{
				pos = [2]int{at[0]+1,at[1]}
			} else{
				pos = [2]int{at[0],at[1]-1}
			}
		default:
			log.Fatal("Piece", piece, "not implemented")

	}
	return pos;
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
	chart := make([][]uint8, 0)
	scanner := bufio.NewScanner(file)
	for scanner.Scan(){
		chart = append(chart, []uint8(scanner.Text()))
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
	validExits := make([][2]int, 0)
	if sPosition[0] > 0{
		if strings.ContainsAny(string(chart[sPosition[0]-1][sPosition[1]]), "7F|"){
			validExits=append(validExits, [2]int{sPosition[0]-1,sPosition[1]})
		}
	}
	if sPosition[1] > 0{
		if strings.ContainsAny(string(chart[sPosition[0]][sPosition[1]-1]), "LF-"){
			validExits=append(validExits, [2]int{sPosition[0],sPosition[1]-1})
		}
	}
	if sPosition[0] < len(chart)-1{
		if strings.ContainsAny(string(chart[sPosition[0]+1][sPosition[1]]), "LJ|"){
			validExits=append(validExits, [2]int{sPosition[0]+1,sPosition[1]})
		}
	}
	if sPosition[1] < len(chart[sPosition[0]])-1{
		if strings.ContainsAny(string(chart[sPosition[0]][sPosition[1]+1]), "J7-"){
			validExits=append(validExits, [2]int{sPosition[0],sPosition[1]+1})
		}
	}
	if len(validExits) != 2{
		log.Fatal("Number different than 2 of valid exits from position S detected")
	}
	path := make([][2]int, 0)
	currentPos := validExits[0]
	lastPosition := sPosition
	stepsTaken := 1
	path = append(path, currentPos)
	for ;currentPos != sPosition;{
		temp := currentPos
		currentPos = walk(chart[currentPos[0]][currentPos[1]], currentPos, lastPosition)
		lastPosition = temp
		stepsTaken++
		path = append(path, currentPos)
	}
	log.Println("Result part 1:", stepsTaken/2)
	// part 2 green theorem for area =>
	// Area = 0.5 * sum (-ydx + xdy) for every step
	// will integrate middle of border line and subtract line width value
	// for every straight line in path increase straight line count by one
	// counter clockwise corner will increase type 1 corner by one
	// clockwise corner will increase type 2 corner by two
	straight := 0
	corner1 := 0
	corner2 := 0
	sum := 0
	chart[sPosition[0]][sPosition[1]] = findSPiece(sPosition, validExits)

	piece := chart[path[0][0]][path[0][1]]
	dx := path[0][1]-path[len(path)-1][1]
	dy := -(path[0][0]-path[len(path)-1][0])
	x := path[0][1]
	y := -path[0][0]
	sum += x*dy - y*dx
	if piece == '|' || piece == '-'{
		straight++
	}

	if (dx > 0 && piece == 'J')||
		(dy > 0 && piece == '7')||
		(dx < 0 && piece == 'F')||
		(dy < 0 && piece == 'L'){
		corner1++
	}

	if (dx > 0 && piece == '7')||
		(dy > 0 && piece == 'F')||
		(dx < 0 && piece == 'L')||
		(dy < 0 && piece == 'J'){
		corner2++
	}
	for i:=1;i<len(path);i++{
		piece = chart[path[i][0]][path[i][1]]
		dx = path[i][1]-path[i-1][1]
		dy = -(path[i][0]-path[i-1][0])
		x = path[i][1]
		y = -path[i][0]
		sum += x*dy - y*dx
		if piece == '|' || piece == '-'{
			straight++
			continue
		}

		if (dx > 0 && piece == 'J')||
			(dy > 0 && piece == '7')||
			(dx < 0 && piece == 'F')||
			(dy < 0 && piece == 'L'){
			corner1++
			continue
		}

		if (dx > 0 && piece == '7')||
			(dy > 0 && piece == 'F')||
			(dx < 0 && piece == 'L')||
			(dy < 0 && piece == 'J'){
			corner2++
			continue
		}
		log.Fatal("unexpected piece delta combination ", dx, dy, piece, path[i], path[i-1])
	}
	sumf := 0.0
	if sum > 0{
		sumf = float64(sum)/2 - (float64(straight)*0.5 + float64(corner1)*0.25 + float64(corner2)*0.75)
	} else if sum < 0{
		sumf = -float64(sum)/2 - (float64(straight)*0.5 + float64(corner1)*0.75 + float64(corner2)*0.25)
	}
	log.Println("Result part 2:", sumf)
	return
}
