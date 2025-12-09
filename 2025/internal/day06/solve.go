package day06

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type MathProblem struct {
	Numbers []int
	Symbol string
}

func SolvePart2() {
	fmt.Println("Solving part 2")

	data, _ := os.ReadFile("inputs/day06.txt")
	dataRows := strings.Split(string(data), "\n")

	problemGrid := [][]rune{}
	for i := 0; i < len(dataRows) - 2; i++ {
		r := []rune(dataRows[i])
		problemGrid = append(problemGrid, r)
	}

	problems := getProblems()
	for i := range problems {
		problems[i].Numbers = nil
	}

	problemIndex := 0
	for j := 0; j < len(dataRows[0]); j++ {
		num := getColumn(problemGrid, j)
		numTrimmed := strings.TrimSpace(string(num))
		if numTrimmed != "" {
			numTrimmedConverted, _ := strconv.Atoi(numTrimmed)
			problems[problemIndex].Numbers = append(problems[problemIndex].Numbers, numTrimmedConverted)
		} else {
			problemIndex++
		}
	}

	totalSum := 0
	for _, problem := range problems {
		switch problem.Symbol {
		case "+":
			for _, num := range problem.Numbers {
				totalSum += num
			}
		case "*": 
			localSum := 1
			for _, num := range problem.Numbers {
				localSum *= num
			}
			totalSum += localSum
		}
	}

	fmt.Println("Total sum: ", totalSum)

}

func getColumn(grid [][]rune, col int) []rune {
    column := make([]rune, len(grid))
    for i := range grid {
        column[i] = grid[i][col]
    }

    return column
}

func SolvePart1() {
	fmt.Println("Solving part 1")

	problems := getProblems()
	fmt.Println(problems[0].Symbol)

	totalSum := 0
	for _, problem := range problems {
		switch problem.Symbol {
		case "+":
			for _, num := range problem.Numbers {
				totalSum += num
			}
		case "*": 
			localSum := 1
			for _, num := range problem.Numbers {
				localSum *= num
			}
			totalSum += localSum
		}
	}

	fmt.Println("Total sum: ", totalSum)
}

func getProblems() []MathProblem {
	data, _ := os.ReadFile("inputs/day06.txt")
	dataRows := strings.Split(string(data), "\n")

	var mathProblems []MathProblem = make([]MathProblem, len(strings.Fields(dataRows[0])))
	for i, row := range dataRows {
		if i == len(dataRows) - 1 {
			continue
		}

		numbers := strings.Fields(row)

		for j, num := range numbers {
			if i == len(dataRows) - 2 {
				mathProblems[j].Symbol = num
			} else {
				n, _ := strconv.Atoi(num)
				mathProblems[j].Numbers = append(mathProblems[j].Numbers, n)
			}

		}
	}

	return mathProblems
}
