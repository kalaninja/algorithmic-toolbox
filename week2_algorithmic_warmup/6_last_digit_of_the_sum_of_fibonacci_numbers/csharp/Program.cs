using System;
using System.Collections.Generic;

namespace FibonacciSumLastDigit
{
    class Program
    {
        static void Main(string[] args)
        {
            // var period = FindPeriod();
            // Console.WriteLine($"{period.Count} -> {string.Join(", ", period.Select(x => x.sum))}");

            var period = new[]
            {
                0, 1, 2, 4, 7, 2, 0, 3, 4, 8, 3, 2, 6, 9, 6, 6, 3, 0, 4, 5, 0, 6, 7, 4, 2, 7, 0, 8, 9, 8, 8, 7, 6, 4, 1,
                6, 8, 5, 4, 0, 5, 6, 2, 9, 2, 2, 5, 8, 4, 3, 8, 2, 1, 4, 6, 1, 8, 0, 9, 0
            };

            var n = long.Parse(Console.ReadLine());
            Console.WriteLine(period[n % 60]);
        }

        /*
        private static IReadOnlyList<(int value, int sum)> FindPeriod()
        {
            var period = new List<(int value, int sum)> {(0, 0), (1, 1)};

            while (true)
            {
                var next = (period[^1].value + period[^2].value) % 10;

                if (next == 1 && period[^1].value == 0)
                {
                    return period.GetRange(0, period.Count - 1);
                }

                period.Add((next, (next + period[^1].sum) % 10));
            }
        }
        */
    }
}