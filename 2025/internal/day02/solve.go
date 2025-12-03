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

	invalidIdsCount := 0
	for _, interval := range intervals {
		for i := interval.Start; i <= interval.End; i++ {
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

func SolvePart2() {
	fmt.Println("Solve part 2")

	data, _ := os.ReadFile("inputs/day02.txt")
	intervalStrings := strings.Split(strings.TrimSpace(string(data)), ",")

	intervals := []Interval{}
	for _, interval := range intervalStrings {
		intervalStartEnd := strings.Split(interval, "-")
		intervalStart, _ := strconv.Atoi(intervalStartEnd[0])
		intervalEnd, _ := strconv.Atoi(intervalStartEnd[1])
		intervals = append(intervals, Interval{ Start: intervalStart, End: intervalEnd })
	}

	invalidIdsCount := 0
	for _, interval := range intervals {
		for i := interval.Start; i <= interval.End; i++ {
			if isValid2(i) {
				invalidIdsCount += i;
			}
		}

	}

	fmt.Print("Invalid id's count: ", invalidIdsCount)
}

func isValid2(id int) bool {
	idAsString := strconv.Itoa(id);
	charCount := utf8.RuneCountInString(idAsString)

	for i := 2; i <= charCount; i++ {
		if charCount % i == 0 {
			stringParts := splitId(idAsString, i)
			if allPartsEqual(stringParts) {
				return true;
			}
		}
	}

	return false
}

func splitId(id string, pieces int) []string {
	idRunes := []rune(id)
    lenRunes := len(idRunes)

	if lenRunes % pieces != 0 {
		panic("Cant divide id")
	}

	sizeOfParts := lenRunes / pieces

	parts := make([]string, pieces)
    for i := range pieces {
        start := i * sizeOfParts
        end := start + sizeOfParts
        parts[i] = string(idRunes[start:end])
    }

    return parts
}

func allPartsEqual(parts []string) bool {
    first := parts[0]
    for _, p := range parts[1:] {
        if p != first {
            return false
        }
    }

    return true
}

