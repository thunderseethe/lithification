(module
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $f32_f32_f32_=>_i32 (func (param f32 f32 f32) (result i32)))
 (type $i32_i32_=>_none (func (param i32 i32)))
 (type $none_=>_none (func))
 (import "env" "memory" (memory $0 (shared 1 100)))
 (data (i32.const 1036) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\"\00\00\00p\00l\00a\00y\00e\00r\00_\00l\00e\00f\00t\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 1100) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e\00\00\00\00\00")
 (data (i32.const 1164) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00s\00t\00u\00b\00.\00t\00s\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 1228) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00$\00\00\00p\00l\00a\00y\00e\00r\00_\00r\00i\00g\00h\00t\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00")
 (data (i32.const 1292) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\1e\00\00\00p\00l\00a\00y\00e\00r\00_\00u\00p\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 1356) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\"\00\00\00p\00l\00a\00y\00e\00r\00_\00d\00o\00w\00n\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 1420) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00(\00\00\00p\00l\00a\00y\00e\00r\00_\00f\00o\00r\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t\00\00\00\00\00")
 (data (i32.const 1484) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00*\00\00\00p\00l\00a\00y\00e\00r\00_\00b\00a\00c\00k\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t\00\00\00")
 (import "common" "is_event" (func $assembly/common/is_event (param i32 i32) (result i32)))
 (import "math" "new_vec3_f32" (func $assembly/math/new_vec3_f32 (param f32 f32 f32) (result i32)))
 (import "math" "append_translation" (func $assembly/math/append_translation (param i32 i32) (result i32)))
 (global $~lib/rt/stub/offset (mut i32) (i32.const 0))
 (export "run" (func $assembly/index/run))
 (export "memory" (memory $0))
 (start $~start)
 (func $assembly/math/Vector3#constructor (param $0 f32) (param $1 f32) (param $2 f32) (result i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  (local $6 i32)
  (local $7 i32)
  global.get $~lib/rt/stub/offset
  local.tee $6
  i32.const 4
  i32.add
  local.tee $3
  i32.const 12
  i32.add
  local.tee $4
  memory.size
  local.tee $5
  i32.const 16
  i32.shl
  i32.const 15
  i32.add
  i32.const -16
  i32.and
  local.tee $7
  i32.gt_u
  if
   local.get $5
   local.get $4
   local.get $7
   i32.sub
   i32.const 65535
   i32.add
   i32.const -65536
   i32.and
   i32.const 16
   i32.shr_u
   local.tee $7
   local.get $5
   local.get $7
   i32.gt_s
   select
   memory.grow
   i32.const 0
   i32.lt_s
   if
    local.get $7
    memory.grow
    i32.const 0
    i32.lt_s
    if
     unreachable
    end
   end
  end
  local.get $4
  global.set $~lib/rt/stub/offset
  local.get $6
  i32.const 12
  i32.store
  local.get $3
  i32.const 0
  i32.store
  local.get $3
  local.get $0
  local.get $1
  local.get $2
  call $assembly/math/new_vec3_f32
  i32.store
  local.get $3
 )
 (func $assembly/index/run (param $0 i32) (param $1 i32)
  (local $2 i32)
  local.get $1
  i32.load
  i32.const 1056
  call $assembly/common/is_event
  if
   f32.const -1
   f32.const 0
   f32.const 0
   call $assembly/math/Vector3#constructor
   local.set $2
   local.get $0
   local.get $0
   i32.load
   local.get $2
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
  local.get $1
  i32.load
  i32.const 1248
  call $assembly/common/is_event
  if
   f32.const 1
   f32.const 0
   f32.const 0
   call $assembly/math/Vector3#constructor
   local.set $2
   local.get $0
   local.get $0
   i32.load
   local.get $2
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
  local.get $1
  i32.load
  i32.const 1312
  call $assembly/common/is_event
  if
   f32.const 0
   f32.const 1
   f32.const 0
   call $assembly/math/Vector3#constructor
   local.set $2
   local.get $0
   local.get $0
   i32.load
   local.get $2
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
  local.get $1
  i32.load
  i32.const 1376
  call $assembly/common/is_event
  if
   f32.const 0
   f32.const -1
   f32.const 0
   call $assembly/math/Vector3#constructor
   local.set $2
   local.get $0
   local.get $0
   i32.load
   local.get $2
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
  local.get $1
  i32.load
  i32.const 1440
  call $assembly/common/is_event
  if
   f32.const 0
   f32.const 0
   f32.const -1
   call $assembly/math/Vector3#constructor
   local.set $2
   local.get $0
   local.get $0
   i32.load
   local.get $2
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
  local.get $1
  i32.load
  i32.const 1504
  call $assembly/common/is_event
  if
   f32.const 0
   f32.const 0
   f32.const 1
   call $assembly/math/Vector3#constructor
   local.set $1
   local.get $0
   local.get $0
   i32.load
   local.get $1
   i32.load
   call $assembly/math/append_translation
   i32.store
  end
 )
 (func $~start
  i32.const 1548
  global.set $~lib/rt/stub/offset
 )
)
