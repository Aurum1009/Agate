using AgateCS.Utils;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AgateCS
{
    /// <summary>
    /// statically-typed instruction set
    /// </summary>
    public enum Instruction
    {
        // operations

        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Add = 1,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Sub = 2,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Mul = 3,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Div = 4,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Rem = 5,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Pow = 6,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        LogAnd = 7,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        LogOr = 8,
        /// <summary>
        /// Operands: reg value, reg dest
        /// </summary>
        LogNot = 9,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Shl = 10,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        Shr = 11,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        BitAnd = 12,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        BitOr = 13,
        /// <summary>
        /// Operands: reg left, reg right, reg dest
        /// </summary>
        BitXor = 14,
        /// <summary>
        /// Operands: reg value, reg dest
        /// </summary>
        BitNot = 15,

        // functions
        /// <summary>
        /// Operands: usize number
        /// </summary>
        BeginFnArgs = 20,
        /// <summary>
        /// Operands: usize name_index, usize num_args, reg dest
        /// </summary>
        Call = 21,
        /// <summary>
        /// Operands: reg value
        /// </summary>
        Return = 22,
    }
}
