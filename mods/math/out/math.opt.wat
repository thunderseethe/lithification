(module
  (type (;0;) (func (param i32 i32 i32 i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param i32)))
  (type (;3;) (func (param i32 i32)))
  (type (;4;) (func (param i32 f32 f32 f32)))
  (func (;0;) (type 0) (param i32 i32 i32 i32)
    local.get 0
    local.get 3
    i64.load align=4
    i64.store offset=28 align=4
    local.get 0
    local.get 2
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 16
    i32.add
    local.get 1
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 36
    i32.add
    local.get 3
    i32.const 8
    i32.add
    i32.load
    i32.store
    local.get 0
    i32.const 8
    i32.add
    local.get 2
    i32.const 8
    i32.add
    i64.load align=4
    i64.store align=4
    local.get 0
    i32.const 24
    i32.add
    local.get 1
    i32.const 8
    i32.add
    i32.load
    i32.store)
  (func (;1;) (type 2) (param i32)
    local.get 0
    i64.const 0
    i64.store offset=16 align=4
    local.get 0
    i64.const 4575657221408423936
    i64.store offset=8 align=4
    local.get 0
    i64.const 0
    i64.store align=4
    local.get 0
    i32.const 32
    i32.add
    i64.const 4575657222473777152
    i64.store align=4
    local.get 0
    i32.const 24
    i32.add
    i64.const 4575657221408423936
    i64.store align=4)
  (func (;2;) (type 3) (param i32 i32)
    (local i32 f32 f32 f32 f32 i32 i32 i32 f32 i32 i32 f32 f32 f32 f32 f32 i32 i32)
    i32.const 1048512
    local.tee 2
    i32.const 60
    i32.add
    i32.const 1065353216
    i32.store
    local.get 2
    i32.const 56
    i32.add
    local.tee 7
    local.get 1
    i32.const 24
    i32.add
    i32.load
    i32.store
    local.get 2
    i32.const 28
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 44
    i32.add
    i32.const 0
    i32.store
    local.get 2
    i32.const 48
    i32.add
    local.tee 8
    local.get 1
    i32.const 16
    i32.add
    i64.load align=4
    i64.store
    local.get 2
    i32.const 8
    i32.add
    local.tee 9
    local.get 1
    f32.load
    local.tee 3
    local.get 1
    f32.load offset=8
    local.tee 10
    f32.mul
    local.tee 4
    local.get 4
    f32.add
    local.tee 13
    local.get 1
    f32.load offset=4
    local.tee 4
    local.get 1
    f32.load offset=12
    local.tee 5
    f32.mul
    local.tee 6
    local.get 6
    f32.add
    local.tee 6
    f32.sub
    f32.store
    local.get 2
    i32.const 16
    i32.add
    local.tee 18
    local.get 3
    local.get 4
    f32.mul
    local.tee 14
    local.get 14
    f32.add
    local.tee 14
    local.get 10
    local.get 5
    f32.mul
    local.tee 15
    local.get 15
    f32.add
    local.tee 15
    f32.sub
    f32.store
    local.get 2
    i32.const 24
    i32.add
    local.tee 11
    local.get 4
    local.get 10
    f32.mul
    local.tee 16
    local.get 16
    f32.add
    local.tee 16
    local.get 3
    local.get 5
    f32.mul
    local.tee 17
    local.get 17
    f32.add
    local.tee 17
    f32.add
    f32.store
    local.get 2
    i32.const 36
    i32.add
    local.get 16
    local.get 17
    f32.sub
    f32.store
    local.get 2
    i32.const 32
    i32.add
    local.tee 12
    local.get 13
    local.get 6
    f32.add
    f32.store
    local.get 2
    i32.const 20
    i32.add
    local.get 4
    local.get 4
    f32.mul
    local.tee 4
    local.get 5
    local.get 5
    f32.mul
    local.tee 5
    local.get 3
    local.get 3
    f32.mul
    local.tee 13
    f32.sub
    local.tee 6
    f32.add
    local.get 10
    local.get 10
    f32.mul
    local.tee 3
    f32.sub
    f32.store
    local.get 2
    i32.const 40
    i32.add
    local.tee 19
    local.get 3
    local.get 6
    local.get 4
    f32.sub
    f32.add
    f32.store
    local.get 2
    i32.const 0
    i32.store offset=12
    local.get 2
    local.get 14
    local.get 15
    f32.add
    f32.store offset=4
    local.get 2
    local.get 13
    local.get 5
    f32.add
    local.get 4
    f32.sub
    local.get 3
    f32.sub
    f32.store
    local.get 0
    i32.const 56
    i32.add
    local.get 7
    i64.load
    i64.store align=4
    local.get 0
    i32.const 32
    i32.add
    local.tee 7
    local.get 12
    i64.load
    i64.store align=4
    local.get 0
    i32.const 24
    i32.add
    local.tee 12
    local.get 11
    i64.load
    i64.store align=4
    local.get 0
    i32.const 8
    i32.add
    local.tee 11
    local.get 9
    i64.load
    i64.store align=4
    local.get 0
    i32.const 48
    i32.add
    local.get 8
    i64.load
    i64.store align=4
    local.get 0
    i32.const 40
    i32.add
    local.tee 8
    local.get 19
    i64.load
    i64.store align=4
    local.get 0
    i32.const 16
    i32.add
    local.tee 9
    local.get 18
    i64.load
    i64.store align=4
    local.get 0
    local.get 2
    i64.load
    i64.store align=4
    local.get 0
    local.get 1
    i32.const 28
    i32.add
    f32.load
    local.tee 3
    local.get 0
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 4
    i32.add
    local.tee 2
    local.get 3
    local.get 2
    f32.load
    f32.mul
    f32.store
    local.get 11
    local.get 3
    local.get 11
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 12
    i32.add
    local.tee 2
    local.get 3
    local.get 2
    f32.load
    f32.mul
    f32.store
    local.get 9
    local.get 1
    i32.const 32
    i32.add
    f32.load
    local.tee 3
    local.get 9
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 20
    i32.add
    local.tee 2
    local.get 3
    local.get 2
    f32.load
    f32.mul
    f32.store
    local.get 12
    local.get 3
    local.get 12
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 28
    i32.add
    local.tee 2
    local.get 3
    local.get 2
    f32.load
    f32.mul
    f32.store
    local.get 7
    local.get 1
    i32.const 36
    i32.add
    f32.load
    local.tee 3
    local.get 7
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 36
    i32.add
    local.tee 1
    local.get 3
    local.get 1
    f32.load
    f32.mul
    f32.store
    local.get 8
    local.get 3
    local.get 8
    f32.load
    f32.mul
    f32.store
    local.get 0
    i32.const 44
    i32.add
    local.tee 0
    local.get 3
    local.get 0
    f32.load
    f32.mul
    f32.store)
  (func (;3;) (type 1) (param i32 i32) (result i32)
    (local i32)
    local.get 0
    local.get 1
    f32.load
    local.get 0
    f32.load offset=16
    f32.add
    f32.store offset=16
    local.get 0
    i32.const 20
    i32.add
    local.tee 2
    local.get 1
    f32.load offset=4
    local.get 2
    f32.load
    f32.add
    f32.store
    local.get 0
    i32.const 24
    i32.add
    local.tee 2
    local.get 1
    f32.load offset=8
    local.get 2
    f32.load
    f32.add
    f32.store
    local.get 0)
  (func (;4;) (type 1) (param i32 i32) (result i32)
    (local f32 f32 f32 f32 f32 f32 f32 f32 f32)
    local.get 0
    local.get 0
    f32.load offset=16
    local.get 1
    f32.load
    local.tee 2
    local.get 0
    f32.load offset=12
    local.tee 7
    local.get 1
    f32.load offset=8
    local.tee 8
    local.get 0
    f32.load offset=4
    local.tee 5
    f32.mul
    local.get 1
    f32.load offset=4
    local.tee 9
    local.get 0
    f32.load offset=8
    local.tee 6
    f32.mul
    f32.sub
    local.tee 3
    local.get 3
    f32.add
    local.tee 10
    f32.mul
    local.get 5
    local.get 9
    local.get 0
    f32.load
    local.tee 3
    f32.mul
    local.get 2
    local.get 5
    f32.mul
    f32.sub
    local.tee 4
    local.get 4
    f32.add
    local.tee 4
    f32.mul
    local.get 6
    local.get 2
    local.get 6
    f32.mul
    local.get 8
    local.get 3
    f32.mul
    f32.sub
    local.tee 2
    local.get 2
    f32.add
    local.tee 2
    f32.mul
    f32.sub
    f32.add
    f32.add
    f32.add
    f32.store offset=16
    local.get 0
    i32.const 20
    i32.add
    local.tee 1
    local.get 1
    f32.load
    local.get 9
    local.get 7
    local.get 2
    f32.mul
    local.get 6
    local.get 10
    f32.mul
    local.get 3
    local.get 4
    f32.mul
    f32.sub
    f32.add
    f32.add
    f32.add
    f32.store
    local.get 0
    i32.const 24
    i32.add
    local.tee 1
    local.get 1
    f32.load
    local.get 8
    local.get 7
    local.get 4
    f32.mul
    local.get 3
    local.get 2
    f32.mul
    local.get 5
    local.get 10
    f32.mul
    f32.sub
    f32.add
    f32.add
    f32.add
    f32.store
    local.get 0)
  (func (;5;) (type 4) (param i32 f32 f32 f32)
    local.get 0
    local.get 3
    f32.store offset=8
    local.get 0
    local.get 2
    f32.store offset=4
    local.get 0
    local.get 1
    f32.store)
  (func (;6;) (type 0) (param i32 i32 i32 i32)
    local.get 0
    local.get 3
    i32.store offset=8
    local.get 0
    local.get 2
    i32.store offset=4
    local.get 0
    local.get 1
    i32.store)
  (memory (;0;) 16)
  (global (;0;) i32 (i32.const 1048576))
  (global (;1;) i32 (i32.const 1048576))
  (export "memory" (memory 0))
  (export "new" (func 0))
  (export "identity" (func 1))
  (export "matrix" (func 2))
  (export "prepend_translation" (func 3))
  (export "append_translation" (func 4))
  (export "new_vec3" (func 5))
  (export "new_vec3_i32" (func 6))
  (export "__data_end" (global 0))
  (export "__heap_base" (global 1)))
