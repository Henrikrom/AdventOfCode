using advent2024;

Console.WriteLine("Advent Of Code 2023");

//Day1.Solve(Advent.ProblemOfTheDay()); 
Advent.NextDay();
//Day2.Solve(Advent.ProblemOfTheDay(day));
Advent.NextDay();
Day3.Solve(Advent.ProblemOfTheDay());

public static class Advent
{
    public static int Day = 1;
    public static string ProblemOfTheDay()
    {
        return $"/home/hbr/advent2024/inputs/day{Day}.txt";
    }

    public static void NextDay() {
        Day++;
    }
}