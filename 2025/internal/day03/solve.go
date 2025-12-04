package day03

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func SolvePart2() {
	fmt.Println("Solving part 2")

	data, _ := os.ReadFile("inputs/day03.txt")
	batteryBanks := strings.Split(strings.TrimSpace(string(data)), "\n")

	joltageSum := 0;
	for _, bank := range batteryBanks {
		joltageSum += highestJoltage2(bank)
	}

	fmt.Println(joltageSum)
}

func SolvePart1() {
	fmt.Println("Solving part 1")

	data, _ := os.ReadFile("inputs/day03.txt")
	batteryBanks := strings.Split(strings.TrimSpace(string(data)), "\n")

	joltageSum := 0;
	for _, bank := range batteryBanks {
		joltageSum += highestJoltage(bank)
	}

	fmt.Println(joltageSum)
}

func highestJoltage2(batteryBank string) int {
	allInts := []int{}
	for _, char := range batteryBank {
		i, _ := strconv.Atoi(string(char))
		allInts = append(allInts, i)
	}

	src := allInts[len(allInts)-12:]
	optimal := append([]int(nil), src...)

	lowestIndexUsed := -1

	highestIndexUsed := -1
	for i := range 12 {
		tmpHeighestIndex := -1
		for j := len(allInts) - 12 + i; j > highestIndexUsed; j-- {
			if j <= lowestIndexUsed {
				fmt.Println("continuing...")
				continue
			}

			joltage := allInts[j]
			if joltage >= optimal[i] {
				optimal[i] = joltage
				tmpHeighestIndex = j
			}
		}

		if tmpHeighestIndex != -1 {
			highestIndexUsed = tmpHeighestIndex
		}
	}

	optimalSliceToString := IntSliceToString(optimal)
	optimalStringToInt, _ := strconv.Atoi(optimalSliceToString)
	return optimalStringToInt
}

func IntSliceToString(nums []int) string {
    var b strings.Builder
    for _, n := range nums {
        b.WriteString(strconv.Itoa(n))
    }
    return b.String()
}

func printIntSlice(slice []int) {
	var stringBuilder strings.Builder
	for _, n := range slice {
		stringBuilder.WriteByte(byte(n + '0'))
	}
	optimalAsString := stringBuilder.String()

	fmt.Println(optimalAsString)

}

func highestJoltage(batteryBank string) int {
	batteryBankChars := []rune(batteryBank)

	highestVoltage := 0
	for i := 0; i < len(batteryBankChars) - 1; i++ {
		for j := i + 1; j < len(batteryBankChars); j++ {
			joltageString := string(batteryBankChars[i]) + string(batteryBankChars[j]);
			voltage, _ := strconv.Atoi(joltageString)
			if (voltage > highestVoltage) {
				highestVoltage = voltage;
			}
		}
	}

	return highestVoltage
}

