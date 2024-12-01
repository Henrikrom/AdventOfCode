using System.Diagnostics;
using advent2024;

public class Day3 : IAdventDay
{
    public static void Solve(string inputPath)
    {
        List<string> inputList = File.ReadAllLines(inputPath).ToList();
        Map map = new Map(inputList);
        

        Part1(map);



        Console.WriteLine("Sum of adjecent parts: " + Part1(map));
    }   
    
    private static int Part1(Map map) {
        for (var y = 1; y < map.Entries[0].Count - 1; y++) {
            for (var x = 1; x < map.Entries[0].Count - 1; x++) {
                if (map.Entries[y][x].Type == MapEntryTypes.NUMBER) {
                    MapEntry previousEntry = map.Entries[y][x - 1];
                    if (previousEntry.Type != MapEntryTypes.NUMBER) {
                        map.Entries[y][x].FullNumber = GetFullNumberAndMarkFirstDigitWithAdjecency(map, x, y);
                    }                   
                }
            }
        }

        int sumOfAdjecentParts = map.Entries.SelectMany(entries => entries)
                                            .Where(entry => entry.Type == MapEntryTypes.NUMBER && entry.IsSymbolAdjecent)
                                            .Sum(entry => entry.FullNumber);
        return sumOfAdjecentParts;
    }

    private static int GetFullNumberAndMarkFirstDigitWithAdjecency(Map map, int xpos, int ypos) {
        if (map.Entries[ypos][xpos].Type != MapEntryTypes.NUMBER) {
            throw new Exception("Why!!");
        }

        if (ypos == 7) {
            Console.Write("");
        }

        int index = 0;
        while (true) {
            if (map.Entries[ypos][xpos - index - 1].Type == MapEntryTypes.NUMBER) {
                index++;
                continue;
            }
            break;
        }

        int startIndex = index;

        bool isAnyPartOfNumberAdjecentToSymbol = IsNumberAdjecentToSymbol(map, xpos, ypos);
        string initialValue = map.Entries[ypos][xpos - index].Value;
        while (true) {
            index--;
            if (map.Entries[ypos][xpos - index].Type == MapEntryTypes.NUMBER) {
                initialValue += map.Entries[ypos][xpos - index].Value;
                if (IsNumberAdjecentToSymbol(map, xpos - index, ypos)) {
                    isAnyPartOfNumberAdjecentToSymbol = true;
                }
            }
            else {
                break;
            }
        }

        if (isAnyPartOfNumberAdjecentToSymbol) {
            map.Entries[ypos][xpos - startIndex].IsSymbolAdjecent = true;
        }      

        return int.Parse(initialValue);
    }

    private static bool IsNumberAdjecentToSymbol(Map map, int xpos, int ypos) {
        List<MapEntry> allAdjecentEntries = new List<MapEntry>();

        MapEntry rightEntry = map.Entries[ypos][xpos + 1];
        MapEntry rightDownEntry = map.Entries[ypos + 1][xpos + 1];
        MapEntry downEntry = map.Entries[ypos + 1][xpos];
        MapEntry leftDownEntry = map.Entries[ypos + 1][xpos - 1];
        MapEntry leftEntry = map.Entries[ypos][xpos - 1];
        MapEntry leftUpEntry = map.Entries[ypos - 1][xpos - 1];
        MapEntry upEntry = map.Entries[ypos - 1][xpos];
        MapEntry rightUpEntry = map.Entries[ypos - 1][xpos + 1];

        allAdjecentEntries.Add(rightEntry);
        allAdjecentEntries.Add(rightDownEntry);
        allAdjecentEntries.Add(downEntry);
        allAdjecentEntries.Add(leftDownEntry);
        allAdjecentEntries.Add(leftEntry);
        allAdjecentEntries.Add(leftUpEntry);
        allAdjecentEntries.Add(upEntry);
        allAdjecentEntries.Add(rightUpEntry);
    
        return allAdjecentEntries.Exists(entry => entry.Type == MapEntryTypes.SYMBOL);
    }

    public class Map 
    {
        public List<List<MapEntry>> Entries = new List<List<MapEntry>>();
        public int Width;
        public int Height;

        public Map(List<string> rawMapLines) {
            Entries = ParseRawMap(rawMapLines);
            Width = Entries[0].Count;
            Height = Entries.Count;
        }

        private List<List<MapEntry>> ParseRawMap(List<string> rawMapLines){
            List<List<MapEntry>> map = new List<List<MapEntry>>();

            List<MapEntry> topBorder = new List<MapEntry>();
            for (int x = 0; x < rawMapLines[0].Length + 2; x++) {
                MapEntry topBorderEntry = new MapEntry("|", x, 0)
                {
                    Type = MapEntryTypes.BORDER
                };
                topBorder.Add(topBorderEntry);
            }

            map.Add(topBorder);

            for (int y = 0; y < rawMapLines.Count; y++) {
                List<MapEntry> mapEntries = new List<MapEntry>();
                MapEntry leftBorder = new MapEntry("|", 0, y - 1)
                {
                    Type = MapEntryTypes.BORDER
                };
                mapEntries.Add(leftBorder);
                
                for (int x = 0; x < rawMapLines[0].Length; x++) {
                    string stringValueInCurrentPosition = rawMapLines[y][x].ToString();

                    MapEntry mapEntry = new MapEntry(stringValueInCurrentPosition, x, y);
                    if (int.TryParse(stringValueInCurrentPosition, out _)) {
                        mapEntry.Type = MapEntryTypes.NUMBER;
                    }
                    else if (stringValueInCurrentPosition == ".") {
                        mapEntry.Type = MapEntryTypes.DOT;
                    }
                    else {
                        mapEntry.Type = MapEntryTypes.SYMBOL;
                    }

                    mapEntries.Add(mapEntry);
                }

                MapEntry rightBorder = new MapEntry("|", rawMapLines[0].Length, y)
                {
                    Type = MapEntryTypes.BORDER
                };
                mapEntries.Add(rightBorder);

                map.Add(mapEntries);
            }

            List<MapEntry> bottomBorder = new List<MapEntry>();
            for (int x = 0; x < rawMapLines[0].Length + 2; x++) {
                MapEntry bottomBorderEntry = new MapEntry("|", x, rawMapLines.Count)
                {
                    Type = MapEntryTypes.BORDER
                };
                bottomBorder.Add(bottomBorderEntry);
            }

            map.Add(bottomBorder);

            return map;
        }
    }

    public class MapEntry
    {
        public int Xpos;
        public int Ypos;
        public string Value;
        public MapEntryTypes Type;
        public bool IsSymbolAdjecent;
        public int FullNumber;

        public MapEntry(string value, int xpos, int ypos) {
            Xpos = xpos;
            Ypos = ypos;
            Value = value;
        }
    }

    public enum MapEntryTypes { NUMBER, DOT, SYMBOL, BORDER }
}