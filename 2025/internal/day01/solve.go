package day01

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func SolvePart2() {
	fmt.Println("Solving part 2")
	lines := getLines()
	
	idx := 50
	zeroIdxCnt := 0

	for _, line := range lines {
		lineNumberAsInt, _ := strconv.Atoi(line[1:])

		zeroIdxCnt += lineNumberAsInt / 100

		switch line[0] {
		case 'L':
			lineNumberMod := lineNumberAsInt % 100
			if lineNumberMod >= idx && idx != 0 {
				zeroIdxCnt++
			}

			numToRotate := 100 - (lineNumberAsInt % 100)
			idx += numToRotate
		case 'R':
			lineNumberMod := lineNumberAsInt % 100
			if lineNumberMod + idx >= 100 && idx != 0 {
				zeroIdxCnt++
			}

			idx += lineNumberMod
		}

		idx %= 100
	}

	println("zeros: ", zeroIdxCnt)
}

func SolvePart1() {
	fmt.Println("Solving part 1")
	lines := getLines()

	idx := 50
	zeroIdxCnt := 0

	for _, line := range lines {
		lineNumberAsInt, _ := strconv.Atoi(line[1:])

		switch line[0] {
		case 'L':
			idx += 100 - lineNumberAsInt % 100
		case 'R':
			idx += lineNumberAsInt % 100
		}

		idx = idx % 100

		if idx == 0 {
			zeroIdxCnt++
		}
	}

	fmt.Println("Zero index count: ", zeroIdxCnt)
}

func getLines() []string {
	data, _ := os.ReadFile("inputs/day01.txt")
	return strings.Split(strings.TrimSpace(string(data)), "\n")
}
