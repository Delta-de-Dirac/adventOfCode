package main

import (
	"bufio"
	"log"
	"os"
	"strconv"
	"strings"
	"slices"
)

// stage 0 => seed
// stage 7 => location

type mapping struct{
	dstI int
	srcI int
	length int
}

type stage struct{
	maps []mapping
}


type almanac struct{
	stages [7]stage
}

func getSeedLocation(seed int, alm almanac) int{
	value := seed
	for i:=0;i<7;i++{
		for _, m := range alm.stages[i].maps{
			if value >= m.srcI && value < m.srcI + m.length{
				value = m.dstI + value - m.srcI
				break
			}
		}
	}
	return value
}

func main(){
	log.Println("Starting...")
	if len(os.Args) != 2 {
		log.Println("Program takes exactly one argument")
		log.Fatal("Usage: day5 <input fileName>")
	}

	// part 1
	file, err := os.Open(os.Args[1])
	if err != nil{
		log.Fatal(err)
	}
	defer file.Close()
	scanner := bufio.NewScanner(file)

	myAlmanac := almanac{}

	seeds := make([]int, 0)
	scanner.Scan()
	seedsLineFields := strings.Fields(scanner.Text())
	for _, field := range seedsLineFields[1:]{
		num, err := strconv.Atoi(field)
		if err != nil{
			log.Fatal(err)
		}
		seeds = append(seeds, num)
	}


	for i:=0;i<7;i++{
		for scanner.Scan(){
			if strings.Contains(scanner.Text(),"map:"){
				break
			}
		}
		myAlmanac.stages[i].maps = make([]mapping, 0)
		for scanner.Scan(){
			if scanner.Text() == ""{
				break
			}
			fields := strings.Fields(scanner.Text())
			dstI, err := strconv.Atoi(fields[0])
			if err != nil{
				log.Fatal(err)
			}
			srcI, err := strconv.Atoi(fields[1])
			if err != nil{
				log.Fatal(err)
			}
			length, err := strconv.Atoi(fields[2])
			if err != nil{
				log.Fatal(err)
			}
			myAlmanac.stages[i].maps = append(myAlmanac.stages[i].maps,
							  mapping{dstI:dstI,
								srcI:srcI,
								length:length,
							})
		}
	}

	// Part 1

	locations := make([]int, 0)
	for _, seed := range seeds{
		locations = append(locations, getSeedLocation(seed,myAlmanac))
	}
	log.Println("Result part 1:", slices.Min(locations))

	// Part 2

	seedIs := make([]int, 0)
	seedLs := make([]int, 0)

	for i:=1;i<len(seedsLineFields);i+=2{
		seedI, err := strconv.Atoi(seedsLineFields[i])
		if err != nil{
			log.Fatal(err)
		}
		seedL, err := strconv.Atoi(seedsLineFields[i+1])
		if err != nil{
			log.Fatal(err)
		}
		seedIs = append(seedIs, seedI)
		seedLs = append(seedLs, seedL)
	}

	minLocation := getSeedLocation(seedIs[0], myAlmanac)

	for i := range seedIs{
		for j:=seedIs[i];j<seedIs[i]+seedLs[i];j++{
			location := getSeedLocation(j,myAlmanac)
			if location < minLocation{
				minLocation = location
			}
		}
	}
	log.Println("Result part 2:", minLocation)
}
