# Rust instruction set

> An operation set is a list of opcodes(AKA an operation), and the _instruction_ set is the set of operation sets. So, you could see an operation set as `Operation[]` and an instruction set as `<Operation[]>[]`

1. NoOp

##### Value Operations

Covers opcodes 1-30

> Operations that produce values with and without input

1. `Add`:
   - Operands: [register dest, register left, register right, operand overflow_mode] (see `overflow_modes.md`for information about how it )
2. `Sub`
3. `Mul`
4. `Div`
5. `Rem`
6. `Pow`
7. `Negate`
8. `LogAnd`
9. `LogOr`
10. `LoxXor`
11. `LogNot`
12. `BitAnd`
13. `BitOr`
14. `BitXor`
15. `BitNot`
16. `Eq`
17. `Ne`
18. `CmpGt`
19. `CmpLt`
20. `CmpGe`
21. `CmpLe`
22. `True`
23. `False`
24. `Null`
25. `BasicCast`

##### Variable & Operative Assignment Operations

Covers opcodes 31-60

> Notes: the Local methods _may_ end up reaching the global scope. However, the Global instructions may increase performance by disabling the recursive rise in scopes.

31. `LocalCreate`
32. `LocalFetch`
33. `LocalAssign`
34. `LocalAddAssign`
35. `LocalSubAssign`
36. `LocalMulAssign`
37. `LocalDivAssign`
38. `LocalRemAssign`
39. `LocalPowAssign`
40. `LocalPreIncrement`
41. `LocalPostIncrement`
42. `LocalPreDecrement`
43. `LocalPostDecrement`
44. `GlobalAddAssign`
45. `GlobalSubAssign`
46. `GlobalMulAssign`
47. `GlobalDivAssign`
48. `GlobalRemAssign`
49. `GlobalPowAssign`
50. `GlobalPreIncrement`
51. `GlobalPostIncrement`
52. `GlobalPreDecrement`
53. `GlobalPostDecrement`

##### Object, Value and Model operations

> Provides the operations to fetch and set object properties (functions are technically properties EXCLUDING static functions), as well as operations to create basic values

45. DefaultInitObject
46. ObjectGet
47. ObjectSet
48. ObjectStaticCall
49. FetchModel
50. ModelStaticCall
51. ShallowCloneValue
52. DeepCloneValue
53. FetchConstant

##### Function & async operations

> Provides the operations to call functions, and async capabilities as well

58. BeginFnArgs
59. Call
60. AsyncCall
61. AwaitResult
62. Return

##### Basic internal functionality

> Provides operations for operations like IO, Threads, Exiting, etc

63. Print
64. PrintLm
65. ErrPrint
66. ErrPrintLn
67. Flush
68. WriteToFile
69. ReadFile
70. ReadLine
71. SpawnThread
72. JoinThread
73. SpawnProcess
74. Exit

##### Jump and Loop operations

> Low-level control flow (Jump/JumpIfFalse = if, Loop = while + for)

75. Jump
76. JumpIfFalse
77. Loop

##### Register operations

> Instructions directly regarding GPR (general purpose registers) and special registers like RET and PASSUP(which holds values passed up by blocks like Rust does except in a VM settting instead of machine code)

78. ClearRegister
79. SwapRegs
80. FetchReturnValue
81. FetchPassUpValue
82. ClearReturnValue
83. ClearPassUpValue

##### Scope operations

> These are instructions that deal with the VM's scopes

84. CreateScope
85. DeleteScope
86. PlacePassUp

##### Tuple and Array operations

> Operations that support the construction of tuple and array values

87. BeginArrayItems
88. BeginTupleItems
89. ConstructArray
90. ConstructTuple
