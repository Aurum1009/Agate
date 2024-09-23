(module
    (import "env" "print_i64" (func $print_i64 (param i64)))

    (func $add_int (param $x i64) (param $y i64) (result i64)
        (local $sum i64)

        local.get $x
        local.get $y
        i64.add

        local.set $sum

        local.get $sum
        call $print_i64

        local.get $sum
        return
    )

    (global $main i64)

    (func $main 
        i64.const 1
        i64.const 2
        call $print_i64
    )
)