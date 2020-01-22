using System;
using System.Linq;

namespace Gcd
{
    class Program
    {
        static void Main(string[] args)
        {
            //Test(2000000000);

            var nums = Console.ReadLine().Split(' ').Select(int.Parse).ToList();
            Console.WriteLine(GcdEuclid(nums[0], nums[1]));
        }

        private static int GcdNaive(int a, int b)
        {
            return Enumerable.Range(1, Math.Min(a, b)).Where(x => a % x == 0 && b % x == 0).Max();
        }

        private static int GcdEuclidRec(int a, int b)
        {
            return b == 0 ? a : GcdEuclidRec(b, a % b);
        }

        private static int GcdEuclid(int a, int b)
        {
            while (true)
            {
                if (b == 0)
                {
                    return a;
                }

                var buff = a;
                a = b;
                b = buff % b;
            }
        }

        private static void Test(int max)
        {
            var rnd = new Random();

            var ok = true;
            while (ok)
            {
                var a = rnd.Next(1, max);
                var b = rnd.Next(1, max);

                ok = GcdNaive(a, b) == GcdEuclid(a, b);
                Console.WriteLine("{0}. a = {1}, b = {2}", ok ? "Ok" : "Fail", a, b);
            }
        }
    }
}