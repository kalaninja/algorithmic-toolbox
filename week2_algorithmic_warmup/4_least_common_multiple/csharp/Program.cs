using System;
using System.Collections.Generic;
using System.Linq;

namespace Lcm
{
    class Program
    {
        static void Main(string[] args)
        {
            // Test(10000000);

            var nums = Console.ReadLine().Split().Select(int.Parse).ToList();
            Console.WriteLine(LcmFormula(nums[0], nums[1]));
        }

        private static long LcmNaive(int a, int b)
        {
            var max = Math.Max(a, b);
            long result = max;
            while (result % a != 0 || result % b != 0)
            {
                result += max;
            }

            return result;
        }

        private static long LcmFormula(int a, int b)
        {
            var mul = (long) a * b;

            while (true)
            {
                if (b == 0)
                {
                    break;
                }

                var buff = a;
                a = b;
                b = buff % b;
            }

            return mul / a;
        }

        private static long LcmFactors(int a, int b)
        {
            var factorsA = Factorize(a);
            var factorsB = Factorize(b);

            foreach (var factor in factorsB)
            {
                int count;
                if (factorsA.TryGetValue(factor.Key, out count))
                {
                    if (factor.Value > count)
                    {
                        factorsA[factor.Key] = factor.Value;
                    }
                }
                else
                {
                    factorsA.Add(factor);
                }
            }

            var result = 1L;
            foreach (var factor in factorsA)
            {
                long pow = factor.Key;
                for (var i = 1; i < factor.Value; i++)
                {
                    pow *= factor.Key;
                }

                result *= pow;
            }

            return result;
        }

        private static IDictionary<int, int> Factorize(int value)
        {
            var factors = new Dictionary<int, int>();

            var count = 0;
            while (value % 2 == 0)
            {
                count++;
                value >>= 1;
            }

            if (count > 0)
            {
                factors.Add(2, count);
            }

            for (var i = 3; i <= Math.Sqrt(value); i += 2)
            {
                count = 0;
                while (value % i == 0)
                {
                    count++;
                    value /= i;
                }

                if (count > 0)
                {
                    factors.Add(i, count);
                }
            }

            if (value > 2)
            {
                factors.Add(value, 1);
            }

            return factors;
        }

        private static void Test(int max)
        {
            var rnd = new Random();

            var ok = true;
            while (ok)
            {
                var a = rnd.Next(1, max);
                var b = rnd.Next(1, max);

                ok = LcmNaive(a, b) == LcmFormula(a, b);
                Console.WriteLine("{0}. a = {1}, b = {2}", ok ? "Ok" : "Fail", a, b);
            }
        }
    }
}