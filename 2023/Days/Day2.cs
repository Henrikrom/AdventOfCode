using System.Data;
using System.Runtime.CompilerServices;

namespace advent2024;

public class Day2 : IAdventDay
{
    public static void Solve(string inputPath)
    {
        List<string> inputList = File.ReadAllLines(inputPath).ToList();

        GameParser gameParser = new ();
        List<Game> games = new List<Game>(); 
        foreach (var line in inputList) {
            games.Add(gameParser.ParseGameLine(line));
        }

        Console.WriteLine("Day 2 Part 1 - Sum of id's: " + Part1(games));
        Console.WriteLine("Day 2 Part 1 - Sum of id's: " + Part2(games));
        
    }

    private static int Part2(List<Game> games) {
        int sum = 0;

        foreach (var game in games) {
            Round minimalRound = game.MinimalAmountOfCubes(game.Rounds);
            int cubePower = 1;
            foreach (var cube in minimalRound.cubes) {
                cubePower *= cube.Count;
            }

            sum += cubePower;
        }

        return sum;
    }

    private static int Part1(List<Game> games) {
        Round compareRound = new ();
        compareRound.AddCube(new Cube { Color = CubeColors.BLUE, Count = 14 });
        compareRound.AddCube(new Cube { Color = CubeColors.RED, Count = 12 });
        compareRound.AddCube(new Cube { Color = CubeColors.GREEN, Count = 13 });

        List<Game> possibleGames = games.Where(game => game.IsGamePossible(compareRound, game.Rounds)).ToList();
        int sumOfGameIds = possibleGames.Sum(game => game.ID);
        
        return sumOfGameIds;   
    }

    public class GameParser {
        public Game ParseGameLine(string rawGameLine) {
            return new Game()
            {
                ID = GetGameId(rawGameLine),
                Rounds = GetRounds(rawGameLine)
            };
        }

        private int GetGameId(string gameText) {
            int indexOfColon = gameText.IndexOf(":");
            string startOfGameLine = gameText.Substring(0, indexOfColon);
            int indexOfSpace = startOfGameLine.IndexOf(" ");
            string idPart = startOfGameLine.Substring(indexOfSpace + 1);

            int.TryParse(idPart.ToString(), out var gameId);
            return gameId;
        }

        private List<Round> GetRounds(string gameText) {
            List<Round> rounds = new ();

            int indexOfColon = gameText.IndexOf(":");
            string roundsPartOfGameLine = gameText.Substring(indexOfColon + 1);

            List<string> roundsListRaw = roundsPartOfGameLine.Split(";").ToList();
            foreach (var round in roundsListRaw) {   
                Round roundParsed = new ();

                List<string> roundInfoRaw = round.Split(",").ToList();
                foreach (var cubeInfo in roundInfoRaw) {
                    Cube cube = ParseCubeInfo(cubeInfo);

                    roundParsed.AddCube(cube);
                }       

                rounds.Add(roundParsed);                   
            }

            return rounds;
        }

        private Cube ParseCubeInfo(string cubeInfo) {
            Cube cube = new ();

            string cubeInfoWithoutFirstSpace = cubeInfo.Substring(1);
            int indexOfSpace = cubeInfoWithoutFirstSpace.IndexOf(" ");
            string numberOfCubesAsString = cubeInfoWithoutFirstSpace.Substring(0, indexOfSpace);

            int.TryParse(numberOfCubesAsString, out var count);

            Enum.TryParse<CubeColors>(cubeInfo.Substring(3).ToUpper(), out var cubeColor);

            cube.Count = count;
            cube.Color = cubeColor;

            return cube;
        }
    }

    public class Game {
        public int ID;
        public List<Round> Rounds = new List<Round>();

        public bool IsGamePossible(Round compareRound, List<Round> gameRounds) {
            bool gameIsPossible = true;

            foreach (var gameRound in gameRounds) {
                foreach (var cubeToPlaywith in compareRound.cubes) {
                    switch(cubeToPlaywith.Color) 
                    {
                    case CubeColors.BLUE:
                        if (compareRound.GetCubeByColor(CubeColors.BLUE).Count < gameRound.GetCubeByColor(CubeColors.BLUE).Count) {
                            gameIsPossible = false;                            
                        }
                        break;
                    case CubeColors.RED:
                        if (compareRound.GetCubeByColor(CubeColors.RED).Count < gameRound.GetCubeByColor(CubeColors.RED).Count) {
                            gameIsPossible = false;                            
                        }
                        break;
                    case CubeColors.GREEN:
                        if (compareRound.GetCubeByColor(CubeColors.GREEN).Count < gameRound.GetCubeByColor(CubeColors.GREEN).Count) {
                            gameIsPossible = false;                            
                        }
                        break;
                    default:
                        break;
                    }
                } 
            }

            return gameIsPossible;
        }

        public Round MinimalAmountOfCubes(List<Round> gameRounds) {
            Round miniumRound = new Round();

            Cube blue = new Cube { Color = CubeColors.BLUE, Count = 0 };
            Cube red = new Cube { Color = CubeColors.RED, Count = 0 };
            Cube green = new Cube { Color = CubeColors.GREEN, Count = 0 };
            foreach (var round in gameRounds) {
                int numberOfBluesInRound = round.GetCubeByColor(CubeColors.BLUE).Count;
                if (numberOfBluesInRound > blue.Count) {
                    blue.Count = numberOfBluesInRound;
                }

                int numberOfRedInRound = round.GetCubeByColor(CubeColors.RED).Count;
                if (numberOfRedInRound > red.Count) {
                    red.Count = numberOfRedInRound;
                }

                int numberOfGreenInRound = round.GetCubeByColor(CubeColors.GREEN).Count;
                if (numberOfGreenInRound > green.Count) {
                    green.Count = numberOfGreenInRound;
                }
            }

            miniumRound.AddCube(blue);
            miniumRound.AddCube(red);
            miniumRound.AddCube(green);

            return miniumRound;
        }
    }

    public class Round {
        public List<Cube> cubes = new List<Cube>();
        
        public void AddCube(Cube cube) {
            cubes.Add(cube);
        }

        public Cube GetCubeByColor(CubeColors cubeColor) {
            Cube? cubeWithCertainColor = cubes.FirstOrDefault(cube => cube.Color == cubeColor);
            if (cubeWithCertainColor is null) {
                return new Cube { Color = cubeColor, Count = 0 }; 
            }
            
            return cubeWithCertainColor;
        }
    }

    public class Cube {
        public int Count;
        public CubeColors Color;
    }

    public enum CubeColors { BLUE, GREEN, RED }
}