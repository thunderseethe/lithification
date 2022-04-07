(module
  (type $t0 (func (param i32)))
  (type $t1 (func (param i32 i32) (result i32)))
  (type $t2 (func (param i32 i32 i32 i32)))
  (type $t3 (func (param i32 i32)))
  (type $t4 (func (param i32 f32 f32 f32)))
  (func $new_global_transform (type $t2) (param $p0 i32) (param $p1 i32) (param $p2 i32) (param $p3 i32)
    (local $l4 f32) (local $l5 f32) (local $l6 f32) (local $l7 f32) (local $l8 f32) (local $l9 f32) (local $l10 f32) (local $l11 f32) (local $l12 f32) (local $l13 f32) (local $l14 f32) (local $l15 f32) (local $l16 f32)
    local.get $p0
    i32.const 56
    i32.add
    local.get $p1
    f32.load offset=8
    f32.store
    local.get $p0
    local.get $p1
    i64.load align=4
    i64.store offset=48
    local.get $p0
    i32.const 36
    i32.add
    local.get $p3
    f32.load offset=8
    local.tee $l10
    local.get $p2
    f32.load offset=4
    local.tee $l5
    local.get $p2
    f32.load offset=8
    local.tee $l11
    local.get $l11
    f32.add
    local.tee $l7
    f32.mul
    local.tee $l8
    local.get $p2
    f32.load
    local.tee $l4
    local.get $l4
    f32.add
    local.tee $l14
    local.get $p2
    f32.load offset=12
    local.tee $l6
    f32.mul
    local.tee $l9
    f32.sub
    f32.mul
    f32.store
    local.get $p0
    local.get $l10
    local.get $l4
    local.get $l7
    f32.mul
    local.tee $l15
    local.get $l6
    local.get $l5
    local.get $l5
    f32.add
    local.tee $l12
    f32.mul
    local.tee $l16
    f32.add
    f32.mul
    f32.store offset=32
    local.get $p0
    i32.const 24
    i32.add
    local.get $p3
    f32.load offset=4
    local.tee $l13
    local.get $l8
    local.get $l9
    f32.add
    f32.mul
    f32.store
    local.get $p0
    local.get $l13
    local.get $l4
    local.get $l12
    f32.mul
    local.tee $l8
    local.get $l6
    local.get $l7
    f32.mul
    local.tee $l9
    f32.sub
    f32.mul
    f32.store offset=16
    local.get $p0
    local.get $p3
    f32.load
    local.tee $l6
    local.get $l15
    local.get $l16
    f32.sub
    f32.mul
    f32.store offset=8
    local.get $p0
    local.get $l6
    local.get $l8
    local.get $l9
    f32.add
    f32.mul
    f32.store offset=4
    local.get $p0
    i32.const 40
    i32.add
    local.get $l10
    f32.const 0x1p+0 (;=1;)
    local.get $l4
    local.get $l14
    f32.mul
    local.tee $l4
    local.get $l5
    local.get $l12
    f32.mul
    local.tee $l5
    f32.add
    f32.sub
    f32.mul
    f32.store
    local.get $p0
    i32.const 20
    i32.add
    local.get $l13
    f32.const 0x1p+0 (;=1;)
    local.get $l4
    local.get $l11
    local.get $l7
    f32.mul
    local.tee $l4
    f32.add
    f32.sub
    f32.mul
    f32.store
    local.get $p0
    local.get $l6
    f32.const 0x1p+0 (;=1;)
    local.get $l5
    local.get $l4
    f32.add
    f32.sub
    f32.mul
    f32.store)
  (func $global_transform_identity (type $t0) (param $p0 i32)
    local.get $p0
    i32.const 56
    i32.add
    i32.const 1048648
    i64.load
    i64.store
    local.get $p0
    i32.const 48
    i32.add
    i32.const 1048640
    i64.load
    i64.store
    local.get $p0
    i32.const 40
    i32.add
    i32.const 1048632
    i64.load
    i64.store
    local.get $p0
    i32.const 32
    i32.add
    i32.const 1048624
    i64.load
    i64.store
    local.get $p0
    i32.const 24
    i32.add
    i32.const 1048616
    i64.load
    i64.store
    local.get $p0
    i32.const 16
    i32.add
    i32.const 1048608
    i64.load
    i64.store
    local.get $p0
    i32.const 8
    i32.add
    i32.const 1048600
    i64.load
    i64.store
    local.get $p0
    i32.const 1048592
    i64.load
    i64.store)
  (func $matrix (type $t3) (param $p0 i32) (param $p1 i32)
    local.get $p0
    i32.const 0
    i32.store offset=12
    local.get $p0
    i32.const 60
    i32.add
    i32.const 1065353216
    i32.store
    local.get $p0
    local.get $p1
    i64.load offset=48
    i64.store offset=48
    local.get $p0
    i32.const 44
    i32.add
    i32.const 0
    i32.store
    local.get $p0
    local.get $p1
    i64.load offset=32
    i64.store offset=32
    local.get $p0
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get $p0
    local.get $p1
    i64.load offset=16
    i64.store offset=16
    local.get $p0
    local.get $p1
    f32.load offset=8
    f32.store offset=8
    local.get $p0
    local.get $p1
    i64.load
    i64.store
    local.get $p0
    i32.const 56
    i32.add
    local.get $p1
    i32.const 56
    i32.add
    f32.load
    f32.store
    local.get $p0
    i32.const 40
    i32.add
    local.get $p1
    i32.const 40
    i32.add
    f32.load
    f32.store
    local.get $p0
    i32.const 24
    i32.add
    local.get $p1
    i32.const 24
    i32.add
    f32.load
    f32.store)
  (func $prepend_translation (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 i32)
    local.get $p0
    local.get $p0
    f32.load offset=48
    local.get $p1
    f32.load
    f32.add
    f32.store offset=48
    local.get $p0
    i32.const 56
    i32.add
    local.tee $l2
    local.get $l2
    f32.load
    local.get $p1
    f32.load offset=8
    f32.add
    f32.store
    local.get $p0
    i32.const 52
    i32.add
    local.tee $l2
    local.get $l2
    f32.load
    local.get $p1
    f32.load offset=4
    f32.add
    f32.store
    local.get $p0)
  (func $append_translation (type $t1) (param $p0 i32) (param $p1 i32) (result i32)
    (local $l2 f32) (local $l3 f32) (local $l4 f32)
    local.get $p0
    local.get $p0
    f32.load offset=48
    local.get $p0
    f32.load offset=32
    local.get $p1
    f32.load offset=8
    local.tee $l2
    f32.mul
    local.get $p0
    f32.load offset=16
    local.get $p1
    f32.load offset=4
    local.tee $l3
    f32.mul
    local.get $p1
    f32.load
    local.tee $l4
    local.get $p0
    f32.load
    f32.mul
    f32.add
    f32.add
    f32.add
    f32.store offset=48
    local.get $p0
    i32.const 56
    i32.add
    local.tee $p1
    local.get $p1
    f32.load
    local.get $l2
    local.get $p0
    i32.const 40
    i32.add
    f32.load
    f32.mul
    local.get $l3
    local.get $p0
    i32.const 24
    i32.add
    f32.load
    f32.mul
    local.get $l4
    local.get $p0
    f32.load offset=8
    f32.mul
    f32.add
    f32.add
    f32.add
    f32.store
    local.get $p0
    i32.const 52
    i32.add
    local.tee $p1
    local.get $p1
    f32.load
    local.get $l2
    local.get $p0
    i32.const 36
    i32.add
    f32.load
    f32.mul
    local.get $l3
    local.get $p0
    i32.const 20
    i32.add
    f32.load
    f32.mul
    local.get $l4
    local.get $p0
    f32.load offset=4
    f32.mul
    f32.add
    f32.add
    f32.add
    f32.store
    local.get $p0)
  (func $new_vec3 (type $t4) (param $p0 i32) (param $p1 f32) (param $p2 f32) (param $p3 f32)
    local.get $p0
    local.get $p3
    f32.store offset=8
    local.get $p0
    local.get $p2
    f32.store offset=4
    local.get $p0
    local.get $p1
    f32.store)
  (func $vec3_x_axis (type $t0) (param $p0 i32)
    local.get $p0
    i32.const 8
    i32.add
    i32.const 1048664
    i32.load
    i32.store
    local.get $p0
    i32.const 1048656
    i64.load align=4
    i64.store align=4)
  (func $vec3_y_axis (type $t0) (param $p0 i32)
    local.get $p0
    i32.const 8
    i32.add
    i32.const 1048676
    i32.load
    i32.store
    local.get $p0
    i32.const 1048668
    i64.load align=4
    i64.store align=4)
  (func $vec3_z_axis (type $t0) (param $p0 i32)
    local.get $p0
    i32.const 8
    i32.add
    i32.const 1048688
    i32.load
    i32.store
    local.get $p0
    i32.const 1048680
    i64.load align=4
    i64.store align=4)
  (memory $memory 17)
  (global $GLOBAL_TRANSFORM_SIZE i32 (i32.const 1048576))
  (global $__data_end i32 (i32.const 1048692))
  (global $__heap_base i32 (i32.const 1048704))
  (export "memory" (memory 0))
  (export "new_global_transform" (func $new_global_transform))
  (export "global_transform_identity" (func $global_transform_identity))
  (export "matrix" (func $matrix))
  (export "prepend_translation" (func $prepend_translation))
  (export "append_translation" (func $append_translation))
  (export "new_vec3" (func $new_vec3))
  (export "vec3_x_axis" (func $vec3_x_axis))
  (export "vec3_y_axis" (func $vec3_y_axis))
  (export "vec3_z_axis" (func $vec3_z_axis))
  (export "GLOBAL_TRANSFORM_SIZE" (global 0))
  (export "__data_end" (global 1))
  (export "__heap_base" (global 2))
  (data $d0 (i32.const 1048576) "@")
  (data $d1 (i32.const 1048594) "\80?")
  (data $d2 (i32.const 1048614) "\80?")
  (data $d3 (i32.const 1048634) "\80?")
  (data $d4 (i32.const 1048658) "\80?")
  (data $d5 (i32.const 1048674) "\80?")
  (data $d6 (i32.const 1048690) "\80?"))
