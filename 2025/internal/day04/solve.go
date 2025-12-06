package day04

import (
	"fmt"
	"os"
	"strings"
)

func SolvePart2() {
	fmt.Println("Solve part 2")

	data, _ := os.ReadFile("inputs/day04.txt")
	diagramRows := strings.Split(strings.TrimSpace(string(data)), "\n")

	diagram := make([][]rune, len(diagramRows))
    for i, diagramRow := range diagramRows {
        diagram[i] = []rune(diagramRow)
    }

	cntBefore := countAts(diagram)
	removeScrolls(diagram)
	cntAfter := countAts(diagram)
	fmt.Println("Removed: ", cntBefore - cntAfter)
}

func removeScrolls(diagram [][]rune) {
	// urintDiagram(diagram)

	rollCnt := 0
	for y := 1; y < len(diagram) - 1; y++ {
		row := diagram[y]
		for x := 1; x < len(row) - 1; x++ {
			if diagram[y][x] != '@' {
				continue
			}

			surround := []rune{}

			l := diagram[y][x - 1]
			if l == '@' || l == 'x' {
				surround = append(surround, l)
			}

			ul := diagram[y - 1][x - 1]
			if ul == '@' || ul == 'x' {
				surround = append(surround, ul)
			}

			u := diagram[y - 1][x]
			if u == '@' || u == 'x' {
				surround = append(surround, u)
			}

			ur := diagram[y - 1][x + 1]
			if ur == '@' || ur == 'x' {
				surround = append(surround, ur)
			}

			r := diagram[y][x + 1]
			if r == '@' || r == 'x' {
				surround = append(surround, r)
			}

			dr := diagram[y + 1][x + 1]
			if dr == '@' || dr == 'x' {
				surround = append(surround, dr)
			}

			d := diagram[y + 1][x]
			if d == '@' || d == 'x' {
				surround = append(surround, d)
			}

			dl := diagram[y + 1][x - 1]
			if dl == '@' || dl == 'x' {
				surround = append(surround, dl)
			}
			
			if len(surround) < 4 {
				diagram[y][x] = '.'
				rollCnt++
			}
		}
	} 

	if rollCnt == 0 {
		return
	}

	removeScrolls(diagram)
}

func countAts(diagram [][]rune) int {
	count := 0

	for i := range diagram {
		for j := range diagram[i] {
			if diagram[i][j] == '@' {
				count++
			}
		}
	}

	return count
}

func SolvePart1() {
	fmt.Println("Solve part 1")

	data, _ := os.ReadFile("inputs/day04.txt")
	diagramRows := strings.Split(strings.TrimSpace(string(data)), "\n")

	diagram := make([][]rune, len(diagramRows))
    for i, diagramRow := range diagramRows {
        diagram[i] = []rune(diagramRow)
    }

	rollCnt := 0
	for y := 1; y < len(diagram) - 1; y++ {
		row := diagram[y]
		for x := 1; x < len(row) - 1; x++ {
			if diagram[y][x] != '@' {
				continue
			}

			surround := []rune{}

			l := diagram[y][x - 1]
			if l == '@' || l == 'x' {
				surround = append(surround, l)
			}

			ul := diagram[y - 1][x - 1]
			if ul == '@' || ul == 'x' {
				surround = append(surround, ul)
			}

			u := diagram[y - 1][x]
			if u == '@' || u == 'x' {
				surround = append(surround, u)
			}

			ur := diagram[y - 1][x + 1]
			if ur == '@' || ur == 'x' {
				surround = append(surround, ur)
			}

			r := diagram[y][x + 1]
			if r == '@' || r == 'x' {
				surround = append(surround, r)
			}

			dr := diagram[y + 1][x + 1]
			if dr == '@' || dr == 'x' {
				surround = append(surround, dr)
			}

			d := diagram[y + 1][x]
			if d == '@' || d == 'x' {
				surround = append(surround, d)
			}

			dl := diagram[y + 1][x - 1]
			if dl == '@' || dl == 'x' {
				surround = append(surround, dl)
			}
			
			if len(surround) < 4 {
				diagram[y][x] = 'x'
				rollCnt++
			}
		}
	} 

	printDiagram(diagram)

	fmt.Println("Roll Count", rollCnt)

}

func printDiagram(diagram [][]rune) {
	for _, row := range diagram {
		fmt.Println(string(row))
	}
}
