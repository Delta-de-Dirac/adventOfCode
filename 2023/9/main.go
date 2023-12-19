package main

import (
	"bufio"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)


func isAllZero(s []int) bool{
	for _, v := range(s){
		if v != 0{
			return false
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
	sum := 0
	for scanner.Scan(){
		lineSum := 0
		fields := strings.Fields(scanner.Text())
		nums := make([]int, 0)
		for _, f := range(fields){
			num, err := strconv.Atoi(f)
			if err!=nil{
				log.Fatal(err)
			}
			nums = append(nums, num)
		}
		lineSum += nums[len(nums)-1]
		diff := make([]int, 0)
		for i := range(nums[:len(nums)-1]){
			diff = append(diff, nums[i+1] - nums[i])
		}
		lineSum += diff[len(diff)-1]
		for ;!isAllZero(diff);{
			n_diff := make([]int, 0)
			for i := range(diff[:len(diff)-1]){
				n_diff = append(n_diff, diff[i+1] - diff[i])
			}
			lineSum += n_diff[len(n_diff)-1]
			diff = n_diff
		}
		sum += lineSum
	}

	log.Println("Result to part 1:", sum)
	// part 2
	file.Seek(io.SeekStart, 0)
	scanner = bufio.NewScanner(file)
	sum = 0
	for scanner.Scan(){
		lineSum := 0
		fields := strings.Fields(scanner.Text())
		nums := make([]int, 0)
		for _, f := range(fields){
			num, err := strconv.Atoi(f)
			if err!=nil{
				log.Fatal(err)
			}
			nums = append(nums, num)
		}
		for i, j := 0, len(nums)-1; i < j; i, j = i+1, j-1 {
			nums[i], nums[j] = nums[j], nums[i]
		}

		lineSum += nums[len(nums)-1]
		diff := make([]int, 0)
		for i := range(nums[:len(nums)-1]){
			diff = append(diff, nums[i+1] - nums[i])
		}
		lineSum += diff[len(diff)-1]
		for ;!isAllZero(diff);{
			n_diff := make([]int, 0)
			for i := range(diff[:len(diff)-1]){
				n_diff = append(n_diff, diff[i+1] - diff[i])
			}
			lineSum += n_diff[len(n_diff)-1]
			diff = n_diff
		}
		sum += lineSum
	}

	log.Println("Result to part 2:", sum)
	return
}
