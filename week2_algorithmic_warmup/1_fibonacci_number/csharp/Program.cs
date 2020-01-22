using System;

namespace Fibonacci
{
    class Program
    {
        static void Main(string[] args)
        {
            // Test();

            var n = uint.Parse(Console.ReadLine());
            Console.WriteLine(FibonacciList(n));
        }

        private static long FibonacciNaive(uint n) =>
            n <= 1 ? n : FibonacciNaive(n - 1) + FibonacciNaive(n - 2);

        private static long FibonacciList(uint n)
        {
            if (n <= 1)
            {
                return n;
            }

            var f = new long[n + 1];
            f[0] = 0;
            f[1] = 1;

            for (var i = 2; i <= n; i++)
            {
                f[i] = f[i - 1] + f[i - 2];
            }

            return f[n];
        }

        private static void Test()
        {
            for (var n = 0U; n <= 45; n++)
            {
                Console.WriteLine("{0}. n = {1}", FibonacciNaive(n) == FibonacciList(n) ? "Ok" : "Fail", n);
            }
        }
    }
}