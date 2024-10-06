namespace AgateCS.Utils
{
    public class Option<T>
    {
        public sealed class None : Option<T>
        {

        }
        public sealed class Some(T Value) : Option<T>
        {
            public T Value { get; set; } = Value;
        }
    }
}
