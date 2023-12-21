package main

import (
	"bufio"
	"log"
	"os"
	_"slices"
	"strconv"
	"strings"
)



var makeGroupMemo map[int]([]uint8)

func makeGroup(size int) []uint8{
	val, ok := makeGroupMemo[size]
	if ok{
		return val
	}
	val = make([]uint8, 0)
	for i:=0;i<size;i++{
		val = append(val, '#')
	}
	makeGroupMemo[size] = val
	return val
}

func testMatch(match []uint8, target[]uint8) bool{
	//log.Printf("{%s}<=>{%s}", match, target)
	if len(match) != len(target){
		log.Fatal("match must have same length as target")
	}
	for i,v:=range(match){
		switch v{
			case '.':
				if target[i] == '#'{
					return false
				}
			case '#':
				if target[i] == '.'{
					return false
				}
			default:
				log.Fatal("you shouldn't be here")
		}
	}
	return true
}

func sliceSum(s []int) int{
	acc := 0
	for _,v:=range(s){
		acc += v
	}
	return acc
}

func recCount(possibleLocations [][]int, groups []int, groupIndex int, startFrom int) int{
	if groupIndex >= len(groups){
		return 1
	}
	acc := 0
	outer:
	for _, pos := range(possibleLocations[groupIndex]){
		for ;pos < startFrom;{
			continue outer
		}
		acc += recCount(possibleLocations, groups, groupIndex+1, pos+1+groups[groupIndex])
	}
	return acc
}

func countValidRecords(record []uint8, group []int) int{
	possibleLocations := [][]int{}
	log.Printf("%s||%v",record, group)
	for _,g := range(group){
		locations := []int{}
		for i:=0;i<len(record)-g+1;i++{
			if !testMatch(makeGroup(g), record[i:i+g]){
				continue
			}
			if i > 0{
				if record[i-1] == '#'{
					continue
				}
			}
			if i+g < len(record)-1{
				if record[i+g] == '#'{
					continue
				}
			}
			locations = append(locations, i)
		}
		possibleLocations = append(possibleLocations, locations)
	}
	//log.Println(possibleLocations)
	res := recCount(possibleLocations,group,0,0)

	log.Println(res)

	return res
}


func main(){
	log.Println("Starting...")

	makeGroupMemo = make(map[int]([]uint8))

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
	records := make([][]uint8, 0)
	groups := make([][]int, 0)
	for scanner.Scan(){
		fields := strings.Fields(scanner.Text())
		records = append(records, []uint8(fields[0]))
		newGroups := make([]int, 0)
		for _, v := range(strings.Split(fields[1], ",")){
			num, err := strconv.Atoi(v)
			if err!=nil{
				log.Fatal("Error parsing groups")
			}
			newGroups = append(newGroups, num)
		}
		groups = append(groups, newGroups)
	}
	sum := 0
	for i := range(records){
		sum += countValidRecords(records[i],groups[i])
	}
	log.Println("Result part 1:", sum)

	// part 2
	//records = [][]uint8{[]uint8("?###????????")}
	//groups = [][]int{[]int{3,2,1}}
	for i := range(records){
		newRecord := make([]uint8,0)
		newGroup := make([]int,0)
		for j:=0;j<5;j++{
			newRecord = append(newRecord, records[i]...)
			if j!=4{
				newRecord = append(newRecord, '?')
			}
			newGroup = append(newGroup, groups[i]...)
		}
		records[i] = newRecord
		groups[i] = newGroup
	}

	sum = 0

	for i := range(records){

		partial := countValidRecords(records[i],groups[i])
		//log.Println(i, partial)
		sum += partial
	}
	log.Println("Result part 2:", sum)

	return
}
