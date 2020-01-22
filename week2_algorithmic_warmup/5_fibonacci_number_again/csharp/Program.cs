using System;
using System.Collections.Generic;
using System.Linq;

namespace FibonacciHuge
{
    class Program
    {
        static void Main(string[] args)
        {
            var nums = Console.ReadLine().Split(' ');
            Console.WriteLine(FibonacciMod(ulong.Parse(nums[0]), uint.Parse(nums[1])));
        }

        private static ulong FibonacciMod(ulong n, uint m)
        {
            if (n <= 1)
            {
                return n;
            }

            var period = new List<ulong> {0, 1};
            var last = 1UL;
            var prev = 0UL;
            while (true)
            {
                var next = (prev + last) % m;
                if (n == (ulong) period.Count)
                {
                    return next;
                }

                if (next == 1 && last == 0)
                {
                    break;
                }

                period.Add(next);

                prev = last;
                last = next;
            }

            return period[(int) (n % ((ulong) period.Count - 1UL))];
        }
    }
}