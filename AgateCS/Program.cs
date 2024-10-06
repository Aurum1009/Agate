namespace AgateCS
{
    public static class Program
    {
        public static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
            Console.Write("[");
            foreach (string arg in args)
            {
                Console.Write(arg);
            }
            Console.WriteLine("]");
        }
    }
}