using System;

namespace FibonacciLastDigit
{
    class Program
    {
        static void Main(string[] args)
        {
            // Test();

            var n = uint.Parse(Console.ReadLine());
            Console.WriteLine(FibonacciOpt(n));
        }

        private static uint FibonacciNaive(uint n)
        {
            return n <= 1 ? n : (FibonacciNaive(n - 1) + FibonacciNaive(n - 2)) % 10;
        }

        private static uint FibonacciOpt(uint n)
        {
            if (n <= 1)
            {
                return n;
            }

            var prev = 0U;
            var curr = 1U;
            for (var i = 2; i <= n; i++)
            {
                var next = (prev + curr) % 10;
                prev = curr;
                curr = next;
            }

            return curr;
        }

        private static void Test()
        {
            var rnd = new Random();

            var ok = true;
            while (ok)
            {
                var n = (uint) rnd.Next(50);
                ok = FibonacciNaive(n) == FibonacciOpt(n);

                Console.WriteLine("{0}. n = {1}", ok ? "Ok" : "Fail", n);
            }
        }
    }
}