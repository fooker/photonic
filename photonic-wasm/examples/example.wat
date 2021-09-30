(module
    (global $t (mut f64) (f64.const 0.0))

    (func (export "init") (param $size i64)
    )

    (func (export "update") (param $duration f64)
        (global.set $t
            (f64.add
                (global.get $t)
                (local.get $duration)
            )
        )
    )

    (func $wrap (param $v f64) (result f64)
        (local.get $v)
        (local.get $v)
        (f64.trunc)
        (f64.sub)
    )

    (func (export "render") (param $index i64) (result f64 f64 f64)
        (call $wrap (f64.mul (f64.convert_i64_u (local.get $index)) (f64.div (global.get $t) (f64.const 13))))
        (call $wrap (f64.mul (f64.convert_i64_u (local.get $index)) (f64.div (global.get $t) (f64.const 17))))
        (call $wrap (f64.mul (f64.convert_i64_u (local.get $index)) (f64.div (global.get $t) (f64.const 23))))
    )
)