using System;
using System.Collections.Generic;

namespace FibonacciSumSquares
{
    class Program
    {
        static void Main(string[] args)
        {
            // var period = FindPeriod();
            // Console.WriteLine($"{period.Count} -> {string.Join(", ", period)}");

            var n = ulong.Parse(Console.ReadLine());
            Console.WriteLine(SumFibonacciSquares(n));
        }

        private static uint SumFibonacciSquares(ulong n)
        {
            var period = new uint[]
            {
                0, 1, 1, 2, 3, 5, 8, 3, 1, 4, 5, 9, 4, 3, 7, 0, 7, 7, 4, 1, 5, 6, 1, 7, 8, 5, 3, 8, 1, 9, 0, 9, 9, 8, 7,
                5, 2, 7, 9, 6, 5, 1, 6, 7, 3, 0, 3, 3, 6, 9, 5, 4, 9, 3, 2, 5, 7, 2, 9, 1
            };

            return (period[n % 60] * period[(n + 1) % 60]) % 10;
        }

        private static List<int> FindPeriod()
        {
            var period = new List<int> {0, 1};

            var prev = 0;
            var curr = 1;
            while (true)
            {
                var next = (curr + prev) % 10;
                if (next == 1 && curr == 0)
                {
                    break;
                }

                period.Add(next);
                prev = curr;
                curr = next;
            }

            return period.GetRange(0, period.Count - 1);
        }
    }
}