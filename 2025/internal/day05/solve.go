package day05

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	Start int
	End int
	StartString string
	EndString string
}

type Id struct {
	Value int
	IsFresh bool
}

type Db struct {
	Ranges []Range
	Ids []Id
}

func SolvePart2() {
	fmt.Println("Solving part 2")

	db := getDb();

	for range 10000 {
		for j := 0; j < len(db.Ranges) - 1; j++ {
			mergeFound := false
			for i := j + 1; i < len(db.Ranges); i++ {
				range1 := db.Ranges[j]
				range2 := db.Ranges[i]
				mergedRange, merged := mergeRange(range1, range2)
				if merged {
					db.Ranges[j] = mergedRange
					db.Ranges = append(db.Ranges[:i], db.Ranges[i+1:]...)
					break
				}
			}

			if mergeFound {
				break
			}
		}
	}

	count := 0
	for _, r := range db.Ranges {
		count += r.End - r.Start + 1
	}

	fmt.Println("Count: ", count)
}

func mergeRange(range1 Range, range2 Range) (Range, bool) {
	merged := false
	if range1.Start >= range2.Start && range1.Start <= range2.End && range1.End >= range2.End {
		range2.End = range1.End
		merged = true
	} else if range2.Start >= range1.Start && range2.Start <= range1.End && range2.End >= range1.End {
		range2.Start = range1.Start
		merged = true
	} else if range2.Start <= range1.Start && range2.End >= range1.End {
		merged = true
	} else if range1.Start <= range2.Start && range1.End >= range2.End {
		range2 = range1
		merged = true
	}

	return range2, merged
} 

func SolvePart1() {
	fmt.Println("Solving part 1")

	db := getDb();

	cnt := 0
	for _, id := range db.Ids {
		for _, r := range db.Ranges {
			if id.Value >= r.Start && id.Value <= r.End {
				id.IsFresh = true
				cnt++
				break
			}
		}
	}

	fmt.Println("Fresh count: ", cnt)
}

func getDb() Db {
	data, _ := os.ReadFile("inputs/day05.txt")

	rangeAndList := strings.Split(strings.TrimSpace(string(data)), "\n\n")

	ranges := strings.Split(strings.TrimSpace(rangeAndList[0]), "\n")
	ids := strings.Split(strings.TrimSpace(rangeAndList[1]), "\n")

	db := Db{}

	for _, r := range ranges {
		startEnd := strings.Split(r, "-")
		ra := Range{}
		ra.StartString = startEnd[0]
		ra.EndString = startEnd[1]
		ra.Start, _ = strconv.Atoi(startEnd[0])
		ra.End, _ = strconv.Atoi(startEnd[1])
		db.Ranges = append(db.Ranges, ra)
	}

	for _, id := range ids {
		idStruct := Id{}
		idAsInt, _ := strconv.Atoi(id)
		idStruct.Value = idAsInt
		db.Ids = append(db.Ids, idStruct)
	}

	return db
}



