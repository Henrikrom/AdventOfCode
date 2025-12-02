package day02

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode/utf8"
)

type Interval struct {
    Start int
    End int
}

func SolvePart1() {
	fmt.Println("Solve part 1")

	data, _ := os.ReadFile("inputs/day02.txt")
	intervalStrings := strings.Split(strings.TrimSpace(string(data)), ",")

	intervals := []Interval{}
	for _, interval := range intervalStrings {
		intervalStartEnd := strings.Split(interval, "-")
		intervalStart, _ := strconv.Atoi(intervalStartEnd[0])
		intervalEnd, _ := strconv.Atoi(intervalStartEnd[1])
		intervals = append(intervals, Interval{ Start: intervalStart, End: intervalEnd })
	}

	if isInvalidId(1188511885) {
		fmt.Println("testing valid")
	}

	invalidIdsCount := 0
	for _, interval := range intervals {
		for i := interval.Start; i <= interval.End; i += 1 {
			// fmt.Println("iterator: ", i)
			if isInvalidId(i) {
				invalidIdsCount += i;
			}
		}

	}

	fmt.Print("Invalid id's count: ", invalidIdsCount)
}

func isInvalidId(id int) bool {
	idAsString := strconv.Itoa(id);
	charCount := utf8.RuneCountInString(idAsString)

	isEvenCharCount := charCount % 2 == 0;
	if isEvenCharCount {
		firstPartOfId := idAsString[:charCount/2]
		lastPartOfId := idAsString[charCount/2:]

		return firstPartOfId == lastPartOfId
	}

	return false
}
