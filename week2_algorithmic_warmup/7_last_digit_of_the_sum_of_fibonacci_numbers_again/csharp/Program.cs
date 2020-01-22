using System;
using System.Linq;

namespace FibonacciPartialSum
{
    class Program
    {
        static void Main(string[] args)
        {
            var numers = Console.ReadLine().Split(' ').Select(long.Parse).ToList();
            Console.WriteLine(FibonacciPartialSum(numers[0], numers[1]));
        }

        private static int FibonacciPartialSum(long m, long n)
        {
            int[] period =
            {
                0, 1, 2, 4, 7, 2, 0, 3, 4, 8, 3, 2, 6, 9, 6, 6, 3, 0, 4, 5, 0, 6, 7, 4, 2, 7, 0, 8, 9, 8, 8, 7, 6, 4, 1,
                6, 8, 5, 4, 0, 5, 6, 2, 9, 2, 2, 5, 8, 4, 3, 8, 2, 1, 4, 6, 1, 8, 0, 9, 0
            };

            if (m % 60 == 0)
            {
                return period[n % 60];
            }

            var result = period[n % 60] - period[m % 60 - 1];
            return result >= 0 ? result : result + 10;
        }
    }
}