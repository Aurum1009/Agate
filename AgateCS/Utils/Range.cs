using System;
using System.Collections.Generic;
using System.Linq;
using System.Numerics;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS.Utils
{
    public class Range<T>(T start, T end) 
        where T: struct, IEquatable<T>
    {
        public T Start = start;
        public T End = end;

        public static Range<int> Empty()
        {
            return new(-1, -1);
        }

        public string ToTokenString() => Start.Equals(End) ? $"{this.Start}" : $"{this.Start}..{this.End}";
    }
}
