(module
 (type $i32_i32_=>_i32 (func (param i32 i32) (result i32)))
 (type $i32_i32_=>_none (func (param i32 i32)))
 (type $f32_f32_f32_=>_i32 (func (param f32 f32 f32) (result i32)))
 (type $i32_i32_i32_i32_=>_none (func (param i32 i32 i32 i32)))
 (type $i32_=>_none (func (param i32)))
 (type $i32_=>_i32 (func (param i32) (result i32)))
 (type $i32_f32_f32_f32_=>_i32 (func (param i32 f32 f32 f32) (result i32)))
 (type $none_=>_none (func))
 (import "env" "memory" (memory $0 (shared 1 100)))
 (data (i32.const 12) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\"\00\00\00p\00l\00a\00y\00e\00r\00_\00l\00e\00f\00t\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 76) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e\00\00\00\00\00")
 (data (i32.const 140) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00s\00t\00u\00b\00.\00t\00s\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 204) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00$\00\00\00p\00l\00a\00y\00e\00r\00_\00r\00i\00g\00h\00t\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00")
 (data (i32.const 268) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\1e\00\00\00p\00l\00a\00y\00e\00r\00_\00u\00p\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 332) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\"\00\00\00p\00l\00a\00y\00e\00r\00_\00d\00o\00w\00n\00_\00s\00t\00a\00r\00t\00\00\00\00\00\00\00\00\00\00\00")
 (data (i32.const 396) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00(\00\00\00p\00l\00a\00y\00e\00r\00_\00f\00o\00r\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t\00\00\00\00\00")
 (data (i32.const 460) "<\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00*\00\00\00p\00l\00a\00y\00e\00r\00_\00b\00a\00c\00k\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t\00\00\00")
 (import "common" "is_event" (func $assembly/common/is_event (param i32 i32) (result i32)))
 (import "math" "new_vec3_f32" (func $assembly/math/new_vec3_f32 (param f32 f32 f32) (result i32)))
 (import "env" "abort" (func $~lib/builtins/abort (param i32 i32 i32 i32)))
 (import "math" "append_translation" (func $assembly/math/append_translation (param i32 i32) (result i32)))
 (global $~lib/rt/stub/startOffset (mut i32) (i32.const 0))
 (global $~lib/rt/stub/offset (mut i32) (i32.const 0))
 (global $~lib/memory/__heap_base i32 (i32.const 524))
 (table $0 1 funcref)
 (elem $0 (i32.const 1))
 (export "run" (func $assembly/index/run))
 (export "memory" (memory $0))
 (start $~start)
 (func $assembly/common/Message#is_event (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  i32.load
  local.get $1
  call $assembly/common/is_event
 )
 (func $assembly/math/Vector3#set:ptr (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store
 )
 (func $~lib/rt/stub/maybeGrowMemory (param $0 i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  (local $5 i32)
  memory.size
  local.set $1
  local.get $1
  i32.const 16
  i32.shl
  i32.const 15
  i32.add
  i32.const 15
  i32.const -1
  i32.xor
  i32.and
  local.set $2
  local.get $0
  local.get $2
  i32.gt_u
  if
   local.get $0
   local.get $2
   i32.sub
   i32.const 65535
   i32.add
   i32.const 65535
   i32.const -1
   i32.xor
   i32.and
   i32.const 16
   i32.shr_u
   local.set $3
   local.get $1
   local.tee $4
   local.get $3
   local.tee $5
   local.get $4
   local.get $5
   i32.gt_s
   select
   local.set $4
   local.get $4
   memory.grow
   i32.const 0
   i32.lt_s
   if
    local.get $3
    memory.grow
    i32.const 0
    i32.lt_s
    if
     unreachable
    end
   end
  end
  local.get $0
  global.set $~lib/rt/stub/offset
 )
 (func $~lib/rt/common/BLOCK#set:mmInfo (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store
 )
 (func $~lib/rt/stub/__alloc (param $0 i32) (result i32)
  (local $1 i32)
  (local $2 i32)
  (local $3 i32)
  (local $4 i32)
  local.get $0
  i32.const 1073741820
  i32.gt_u
  if
   i32.const 96
   i32.const 160
   i32.const 33
   i32.const 29
   call $~lib/builtins/abort
   unreachable
  end
  global.get $~lib/rt/stub/offset
  local.set $1
  global.get $~lib/rt/stub/offset
  i32.const 4
  i32.add
  local.set $2
  local.get $0
  local.set $3
  local.get $3
  i32.const 4
  i32.add
  i32.const 15
  i32.add
  i32.const 15
  i32.const -1
  i32.xor
  i32.and
  i32.const 4
  i32.sub
  local.set $4
  local.get $2
  local.get $4
  i32.add
  call $~lib/rt/stub/maybeGrowMemory
  local.get $1
  local.get $4
  call $~lib/rt/common/BLOCK#set:mmInfo
  local.get $2
 )
 (func $assembly/math/Vector3#constructor (param $0 i32) (param $1 f32) (param $2 f32) (param $3 f32) (result i32)
  local.get $0
  i32.eqz
  if
   i32.const 4
   call $~lib/rt/stub/__alloc
   local.set $0
  end
  local.get $0
  i32.const 0
  call $assembly/math/Vector3#set:ptr
  local.get $0
  local.get $1
  local.get $2
  local.get $3
  call $assembly/math/new_vec3_f32
  call $assembly/math/Vector3#set:ptr
  local.get $0
 )
 (func $assembly/math/GlobalTransform#set:ptr (param $0 i32) (param $1 i32)
  local.get $0
  local.get $1
  i32.store
 )
 (func $assembly/math/GlobalTransform#append_translation (param $0 i32) (param $1 i32) (result i32)
  local.get $0
  local.get $0
  i32.load
  local.get $1
  i32.load
  call $assembly/math/append_translation
  call $assembly/math/GlobalTransform#set:ptr
  local.get $0
 )
 (func $assembly/index/run (param $0 i32) (param $1 i32)
  local.get $1
  i32.const 32
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const -1
   f32.const 0
   f32.const 0
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
  local.get $1
  i32.const 224
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const 1
   f32.const 0
   f32.const 0
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
  local.get $1
  i32.const 288
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const 0
   f32.const 1
   f32.const 0
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
  local.get $1
  i32.const 352
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const 0
   f32.const -1
   f32.const 0
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
  local.get $1
  i32.const 416
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const 0
   f32.const 0
   f32.const -1
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
  local.get $1
  i32.const 480
  call $assembly/common/Message#is_event
  if
   local.get $0
   i32.const 0
   f32.const 0
   f32.const 0
   f32.const 1
   call $assembly/math/Vector3#constructor
   call $assembly/math/GlobalTransform#append_translation
   drop
  end
 )
 (func $~start
  global.get $~lib/memory/__heap_base
  i32.const 4
  i32.add
  i32.const 15
  i32.add
  i32.const 15
  i32.const -1
  i32.xor
  i32.and
  i32.const 4
  i32.sub
  global.set $~lib/rt/stub/startOffset
  global.get $~lib/rt/stub/startOffset
  global.set $~lib/rt/stub/offset
 )
)
