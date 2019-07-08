using System;
using System.Collections.Generic;
using System.Linq;

namespace MaxPairwiseProduct
{
    class Program
    {
        static void Main(string[] args)
        {
            var _ = int.Parse(Console.ReadLine());
            var numbers = Console.ReadLine()
                ?.Split(new[] {' '}, StringSplitOptions.RemoveEmptyEntries)
                .Select(int.Parse)
                .ToArray();

            Console.WriteLine(GetMaxPairwiseProduct(numbers));
        }

        private static long GetMaxPairwiseProduct(IEnumerable<int> numbers)
        {
            var max = new long[2];

            foreach (var number in numbers)
            {
                if (number >= max[0])
                {
                    max[1] = max[0];
                    max[0] = number;
                }

                else if (number > max[1])
                {
                    max[1] = number;
                }
            }

            return max[0] * max[1];
        }
    }
}