using System.Collections.Concurrent;
using System.IO.Compression;

namespace advent2024;

public class Day1 : IAdventDay
{
    public static void Solve(string inputPath)
    {
        List<string> inputList = File.ReadAllLines(inputPath).ToList();

        Part1(inputList);
        Part2(inputList);
    }

    public enum Numbers { one = 1, two, three, four, five, six, seven, eight, nine }
    private static void Part2(List<string> parsedInput)
    {
        List<string> reWrittenLines = new List<string>();
        foreach (var x in parsedInput) {
            reWrittenLines.Add(ReplaceStringWithNumber(x));
        }

        Part1(reWrittenLines);
    }

    private static string ReplaceStringWithNumber(string lineInText) {
        for (var i = 1; i < 10; i++) {
            var enumValue = (Numbers)i;
            lineInText = lineInText.Replace(enumValue.ToString(), $"{enumValue}{i}{enumValue}");
        }

        return lineInText;
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