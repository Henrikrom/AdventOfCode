using advent2024;

Console.WriteLine("Advent Of Code 2023");

int day = 1;

//Day1.Solve(Advent.ProblemOfTheDay(day)); 
day++;
Day2.Solve(Advent.ProblemOfTheDay(day));


public static class Advent
{
    public static string ProblemOfTheDay(int day)
    {
        return $"/home/hbr/advent2024/inputs/day{day}.txt";
    }
}