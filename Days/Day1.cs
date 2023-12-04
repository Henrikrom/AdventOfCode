using System.Collections.Concurrent;

namespace advent2024;

public class Day1 : IAdventDay
{
    public static void Solve(string inputPath)
    {
        List<string> inputList = File.ReadAllLines(inputPath).ToList();

        Part1(inputList);
    }

    private static void Part2(List<string> parsedInput)
    {
        List<string> digits = new();

        foreach (var line in parsedInput)
        {
            int firstNumber = -1;
            int lastNumber = -1;

            foreach (var x in line)
            {
                if (int.TryParse(x.ToString(), out var intValue))
                {
                    if (firstNumber == -1)
                    {
                        firstNumber = intValue;
                    }

                    lastNumber = intValue;
                }
            }

            string numberAsString = firstNumber.ToString() + lastNumber;
            digits.Add(numberAsString);
        }

        int sum = 0;

        foreach (var digit in digits)
        {
            sum += int.Parse(digit);
        }
        
        Console.WriteLine("Day 1 solution (part 2): " + sum);
    }
    private static void Part1(List<string> parsedInput)
    {
        List<string> digits = new();

        foreach (var line in parsedInput)
        {
            int firstNumber = -1;
            int lastNumber = -1;

            foreach (var x in line)
            {
                if (int.TryParse(x.ToString(), out var intValue))
                {
                    if (firstNumber == -1)
                    {
                        firstNumber = intValue;
                    }

                    lastNumber = intValue;
                }
            }

            string numberAsString = firstNumber.ToString() + lastNumber;
            digits.Add(numberAsString);
        }

        int sum = 0;

        foreach (var digit in digits)
        {
            sum += int.Parse(digit);
        }
        
        Console.WriteLine("Day 1 solution (part 1): " + sum);
    }
}