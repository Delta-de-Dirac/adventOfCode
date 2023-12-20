package main

import (
	"bufio"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)


func validateRecord(record []uint8, groups []int) bool{
	if slices.Contains(record, '?'){
		log.Fatal("No ? in validate! ", record)
		return false
	}
	fields := strings.FieldsFunc(string(record), func(r rune) bool {
		return (r == '.')
	})
	if len(fields) != len(groups){
		return false
	}
	for i := range(fields){
		if len(fields[i]) != groups[i]{
			return false
		}
	}
	return true
}

func countValidRecords(record []uint8, groups []int) int{
	if !slices.Contains(record, '?'){
		if validateRecord(record, groups){
			return 1
		}
		return 0
	}
	modPos := slices.Index(record, '?')
	subdot := slices.Clone(record)
	subdot[modPos] = '.'
	subhash := slices.Clone(record)
	subhash[modPos] = '#'


	subdotValid := true
	subhashValid := true
	if slices.Contains(subdot, '?'){
		subdotValid = validatePartial(subdot, groups)
		subhashValid = validatePartial(subhash, groups)
	}
	switch [2]bool{subdotValid, subhashValid}{
		case [2]bool{true, true}:
			return countValidRecords(subdot, groups) + countValidRecords(subhash,groups)
		case [2]bool{false, true}:
			return countValidRecords(subhash,groups)
		case [2]bool{true, false}:
			return countValidRecords(subdot, groups)
		case [2]bool{false, false}:
			return 0
		default:
			log.Fatal("Cannot be here")
	}
	log.Fatal("Cannot be here")
	return 0

	//return countValidRecords(subdot, groups) + countValidRecords(subhash,groups)
}

func validatePartial(record []uint8, groups []int) bool{
	if !slices.Contains(record, '?'){
		log.Fatal("Can only validate partial if has ? unknown")
	}
	currentGroup := 0
	lookingForGroup := 0
	for i,v:=range(record){
		if v == '?'{
			sizeAvailable := len(record) - i
			sizeNeeded:=0
			sizeNeeded+=(currentGroup - groups[lookingForGroup])
			lookingForGroup++
			for ;lookingForGroup<len(groups);{
				sizeNeeded+=1+groups[lookingForGroup]
				lookingForGroup++
			}
			if sizeNeeded > sizeAvailable{
				return false
			}
			break
		}
		if v == '#'{
			currentGroup++
			if currentGroup > groups[lookingForGroup]{
				return false
			}
			continue
		}
		if v == '.'{
			if currentGroup == 0{
				continue
			}
			if currentGroup != groups[lookingForGroup]{
				return false
			}
			currentGroup = 0
			lookingForGroup++
			if lookingForGroup >= len(groups){
				if slices.Contains(record[i:], '#'){
					return false
				}
				break
			}
		}
	}
	return true
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
		log.Println(i)
		sum += countValidRecords(records[i],groups[i])
	}
	log.Println("Result part 2:", sum)

	return
}
