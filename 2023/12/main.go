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
	return countValidRecords(subdot, groups) + countValidRecords(subhash,groups)
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
	return
}
