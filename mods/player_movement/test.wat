(module
  (type (;0;) (func (param i32 i32)))
  (type (;1;) (func (param i32 i32) (result i32)))
  (type (;2;) (func (param f32 f32 f32) (result i32)))
  (type (;3;) (func (param i32 i32 i32 i32)))
  (type (;4;) (func (param i32 i32 i32)))
  (type (;5;) (func (param i32) (result i32)))
  (import "common" "is_event" (func (;0;) (type 1)))
  (import "math" "new_vec3_f32" (func (;1;) (type 2)))
  (import "env" "abort" (func (;2;) (type 3)))
  (import "math" "append_translation" (func (;3;) (type 1)))
  (func (;4;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32)
    local.get 1
    i32.load
    local.tee 2
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 268
      i32.const 14
      call 2
      unreachable
    end
    local.get 2
    i32.const -4
    i32.and
    local.tee 2
    i32.const 12
    i32.lt_u
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 270
      i32.const 14
      call 2
      unreachable
    end
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
    else
      i32.const 31
      local.get 2
      i32.const 1073741820
      local.get 2
      i32.const 1073741820
      i32.lt_u
      select
      local.tee 2
      i32.clz
      i32.sub
      local.tee 3
      i32.const 7
      i32.sub
      local.set 4
      local.get 2
      local.get 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
    end
    local.tee 3
    i32.const 16
    i32.lt_u
    local.get 4
    i32.const 23
    i32.lt_u
    i32.and
    i32.eqz
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 284
      i32.const 14
      call 2
      unreachable
    end
    local.get 1
    i32.load offset=8
    local.set 2
    local.get 1
    i32.load offset=4
    local.tee 5
    if  ;; label = @1
      local.get 5
      local.get 2
      i32.store offset=8
    end
    local.get 2
    if  ;; label = @1
      local.get 2
      local.get 5
      i32.store offset=4
    end
    local.get 3
    local.get 4
    i32.const 4
    i32.shl
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=96
    local.get 1
    i32.eq
    if  ;; label = @1
      local.get 3
      local.get 4
      i32.const 4
      i32.shl
      i32.add
      i32.const 2
      i32.shl
      local.get 0
      i32.add
      local.get 2
      i32.store offset=96
      local.get 2
      i32.eqz
      if  ;; label = @2
        local.get 4
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        local.tee 2
        i32.load offset=4
        i32.const -2
        local.get 3
        i32.rotl
        i32.and
        local.set 1
        local.get 2
        local.get 1
        i32.store offset=4
        local.get 1
        i32.eqz
        if  ;; label = @3
          local.get 0
          local.get 0
          i32.load
          i32.const -2
          local.get 4
          i32.rotl
          i32.and
          i32.store
        end
      end
    end)
  (func (;5;) (type 0) (param i32 i32)
    (local i32 i32 i32 i32 i32)
    local.get 1
    i32.eqz
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 201
      i32.const 14
      call 2
      unreachable
    end
    local.get 1
    i32.load
    local.tee 2
    i32.const 1
    i32.and
    i32.eqz
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 203
      i32.const 14
      call 2
      unreachable
    end
    local.get 1
    i32.const 4
    i32.add
    local.get 1
    i32.load
    i32.const -4
    i32.and
    i32.add
    local.tee 3
    i32.load
    local.tee 5
    i32.const 1
    i32.and
    if  ;; label = @1
      local.get 0
      local.get 3
      call 4
      local.get 1
      local.get 2
      i32.const 4
      i32.add
      local.get 5
      i32.const -4
      i32.and
      i32.add
      local.tee 2
      i32.store
      local.get 1
      i32.const 4
      i32.add
      local.get 1
      i32.load
      i32.const -4
      i32.and
      i32.add
      local.tee 3
      i32.load
      local.set 5
    end
    local.get 2
    i32.const 2
    i32.and
    if  ;; label = @1
      local.get 1
      i32.const 4
      i32.sub
      i32.load
      local.tee 1
      i32.load
      local.tee 6
      i32.const 1
      i32.and
      i32.eqz
      if  ;; label = @2
        i32.const 0
        i32.const 1120
        i32.const 221
        i32.const 16
        call 2
        unreachable
      end
      local.get 0
      local.get 1
      call 4
      local.get 1
      local.get 6
      i32.const 4
      i32.add
      local.get 2
      i32.const -4
      i32.and
      i32.add
      local.tee 2
      i32.store
    end
    local.get 3
    local.get 5
    i32.const 2
    i32.or
    i32.store
    local.get 2
    i32.const -4
    i32.and
    local.tee 2
    i32.const 12
    i32.lt_u
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 233
      i32.const 14
      call 2
      unreachable
    end
    local.get 2
    local.get 1
    i32.const 4
    i32.add
    i32.add
    local.get 3
    i32.ne
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 234
      i32.const 14
      call 2
      unreachable
    end
    local.get 3
    i32.const 4
    i32.sub
    local.get 1
    i32.store
    local.get 2
    i32.const 256
    i32.lt_u
    if (result i32)  ;; label = @1
      local.get 2
      i32.const 4
      i32.shr_u
    else
      i32.const 31
      local.get 2
      i32.const 1073741820
      local.get 2
      i32.const 1073741820
      i32.lt_u
      select
      local.tee 2
      i32.clz
      i32.sub
      local.tee 3
      i32.const 7
      i32.sub
      local.set 4
      local.get 2
      local.get 3
      i32.const 4
      i32.sub
      i32.shr_u
      i32.const 16
      i32.xor
    end
    local.tee 2
    i32.const 16
    i32.lt_u
    local.get 4
    i32.const 23
    i32.lt_u
    i32.and
    i32.eqz
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 251
      i32.const 14
      call 2
      unreachable
    end
    local.get 2
    local.get 4
    i32.const 4
    i32.shl
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    i32.load offset=96
    local.set 3
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    local.get 3
    i32.store offset=8
    local.get 3
    if  ;; label = @1
      local.get 3
      local.get 1
      i32.store offset=4
    end
    local.get 2
    local.get 4
    i32.const 4
    i32.shl
    i32.add
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    local.get 1
    i32.store offset=96
    local.get 0
    local.get 0
    i32.load
    i32.const 1
    local.get 4
    i32.shl
    i32.or
    i32.store
    local.get 4
    i32.const 2
    i32.shl
    local.get 0
    i32.add
    local.tee 0
    local.get 0
    i32.load offset=4
    i32.const 1
    local.get 2
    i32.shl
    i32.or
    i32.store offset=4)
  (func (;6;) (type 4) (param i32 i32 i32)
    (local i32 i32)
    local.get 1
    local.get 2
    i32.gt_u
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 377
      i32.const 14
      call 2
      unreachable
    end
    local.get 1
    i32.const 19
    i32.add
    i32.const -16
    i32.and
    i32.const 4
    i32.sub
    local.set 1
    local.get 0
    i32.load offset=1568
    local.tee 3
    if  ;; label = @1
      local.get 1
      local.get 3
      i32.const 4
      i32.add
      i32.lt_u
      if  ;; label = @2
        i32.const 0
        i32.const 1120
        i32.const 384
        i32.const 16
        call 2
        unreachable
      end
      local.get 3
      local.get 1
      i32.const 16
      i32.sub
      i32.eq
      if  ;; label = @2
        local.get 3
        i32.load
        local.set 4
        local.get 1
        i32.const 16
        i32.sub
        local.set 1
      end
    else
      local.get 1
      local.get 0
      i32.const 1572
      i32.add
      i32.lt_u
      if  ;; label = @2
        i32.const 0
        i32.const 1120
        i32.const 397
        i32.const 5
        call 2
        unreachable
      end
    end
    local.get 2
    i32.const -16
    i32.and
    local.get 1
    i32.sub
    local.tee 2
    i32.const 20
    i32.lt_u
    if  ;; label = @1
      return
    end
    local.get 1
    local.get 4
    i32.const 2
    i32.and
    local.get 2
    i32.const 8
    i32.sub
    local.tee 2
    i32.const 1
    i32.or
    i32.or
    i32.store
    local.get 1
    i32.const 0
    i32.store offset=4
    local.get 1
    i32.const 0
    i32.store offset=8
    local.get 2
    local.get 1
    i32.const 4
    i32.add
    i32.add
    local.tee 2
    i32.const 2
    i32.store
    local.get 0
    local.get 2
    i32.store offset=1568
    local.get 0
    local.get 1
    call 5)
  (func (;7;) (type 5) (param i32) (result i32)
    (local i32 i32)
    local.get 0
    i32.load offset=4
    local.tee 1
    if (result i32)  ;; label = @1
      local.get 1
      i32.ctz
      i32.const 2
      i32.shl
      local.get 0
      i32.add
      i32.load offset=96
    else
      local.get 0
      i32.load
      i32.const -2
      i32.and
      local.tee 1
      if (result i32)  ;; label = @2
        local.get 1
        i32.ctz
        local.tee 1
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        i32.load offset=4
        local.tee 2
        i32.eqz
        if  ;; label = @3
          i32.const 0
          i32.const 1120
          i32.const 343
          i32.const 18
          call 2
          unreachable
        end
        local.get 2
        i32.ctz
        local.get 1
        i32.const 4
        i32.shl
        i32.add
        i32.const 2
        i32.shl
        local.get 0
        i32.add
        i32.load offset=96
      else
        i32.const 0
      end
    end)
  (func (;8;) (type 2) (param f32 f32 f32) (result i32)
    (local i32 i32 i32 i32)
    global.get 0
    i32.eqz
    if  ;; label = @1
      memory.size
      local.tee 4
      i32.const 0
      i32.le_s
      if (result i32)  ;; label = @2
        i32.const 1
        local.get 4
        i32.sub
        memory.grow
        i32.const 0
        i32.lt_s
      else
        i32.const 0
      end
      if  ;; label = @2
        unreachable
      end
      i32.const 17936
      i32.const 0
      i32.store
      i32.const 19504
      i32.const 0
      i32.store
      loop  ;; label = @2
        local.get 3
        i32.const 23
        i32.lt_u
        if  ;; label = @3
          local.get 3
          i32.const 2
          i32.shl
          i32.const 17936
          i32.add
          i32.const 0
          i32.store offset=4
          i32.const 0
          local.set 4
          loop  ;; label = @4
            local.get 4
            i32.const 16
            i32.lt_u
            if  ;; label = @5
              local.get 4
              local.get 3
              i32.const 4
              i32.shl
              i32.add
              i32.const 2
              i32.shl
              i32.const 17936
              i32.add
              i32.const 0
              i32.store offset=96
              local.get 4
              i32.const 1
              i32.add
              local.set 4
              br 1 (;@4;)
            end
          end
          local.get 3
          i32.const 1
          i32.add
          local.set 3
          br 1 (;@2;)
        end
      end
      i32.const 17936
      i32.const 19508
      memory.size
      i32.const 16
      i32.shl
      call 6
      i32.const 17936
      global.set 0
    end
    global.get 0
    local.tee 4
    call 7
    local.tee 3
    i32.eqz
    if  ;; label = @1
      memory.size
      local.tee 3
      i32.const 4
      local.get 4
      i32.load offset=1568
      local.get 3
      i32.const 16
      i32.shl
      i32.const 4
      i32.sub
      i32.ne
      i32.shl
      i32.const 2147418102
      i32.sub
      i32.const -65536
      i32.and
      i32.const 16
      i32.shr_u
      local.tee 5
      local.get 3
      local.get 5
      i32.gt_s
      select
      memory.grow
      i32.const 0
      i32.lt_s
      if  ;; label = @2
        local.get 5
        memory.grow
        i32.const 0
        i32.lt_s
        if  ;; label = @3
          unreachable
        end
      end
      local.get 4
      local.get 3
      i32.const 16
      i32.shl
      memory.size
      i32.const 16
      i32.shl
      call 6
      local.get 4
      call 7
      local.tee 3
      i32.eqz
      if  ;; label = @2
        i32.const 0
        i32.const 1120
        i32.const 496
        i32.const 16
        call 2
        unreachable
      end
    end
    local.get 3
    i32.load
    i32.const -4
    i32.and
    i32.const 12
    i32.lt_u
    if  ;; label = @1
      i32.const 0
      i32.const 1120
      i32.const 498
      i32.const 14
      call 2
      unreachable
    end
    local.get 4
    local.get 3
    call 4
    local.get 3
    i32.load
    local.tee 5
    i32.const -4
    i32.and
    i32.const 12
    i32.sub
    local.tee 6
    i32.const 16
    i32.ge_u
    if  ;; label = @1
      local.get 3
      local.get 5
      i32.const 2
      i32.and
      i32.const 12
      i32.or
      i32.store
      local.get 3
      i32.const 16
      i32.add
      local.tee 5
      local.get 6
      i32.const 4
      i32.sub
      i32.const 1
      i32.or
      i32.store
      local.get 4
      local.get 5
      call 5
    else
      local.get 3
      local.get 5
      i32.const -2
      i32.and
      i32.store
      local.get 3
      i32.const 4
      i32.add
      local.get 3
      i32.load
      i32.const -4
      i32.and
      i32.add
      local.tee 4
      local.get 4
      i32.load
      i32.const -3
      i32.and
      i32.store
    end
    local.get 3
    i32.const 4
    i32.add
    local.tee 3
    i32.const 0
    i32.store
    local.get 3
    local.get 0
    local.get 1
    local.get 2
    call 1
    i32.store
    local.get 3)
  (func (;9;) (type 0) (param i32 i32)
    (local i32)
    global.get 1
    i32.const 4
    i32.sub
    global.set 1
    global.get 1
    i32.const 1548
    i32.lt_s
    if  ;; label = @1
      i32.const 17952
      i32.const 18000
      i32.const 1
      i32.const 1
      call 2
      unreachable
    end
    global.get 1
    local.tee 2
    i32.const 0
    i32.store
    local.get 2
    i32.const 1056
    i32.store
    local.get 1
    i32.load
    i32.const 1056
    call 0
    if  ;; label = @1
      f32.const -0x1p+0 (;=-1;)
      f32.const 0x0p+0 (;=0;)
      f32.const 0x0p+0 (;=0;)
      call 8
      local.set 2
      local.get 0
      local.get 0
      i32.load
      local.get 2
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 1248
    i32.store
    local.get 1
    i32.load
    i32.const 1248
    call 0
    if  ;; label = @1
      f32.const 0x1p+0 (;=1;)
      f32.const 0x0p+0 (;=0;)
      f32.const 0x0p+0 (;=0;)
      call 8
      local.set 2
      local.get 0
      local.get 0
      i32.load
      local.get 2
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 1312
    i32.store
    local.get 1
    i32.load
    i32.const 1312
    call 0
    if  ;; label = @1
      f32.const 0x0p+0 (;=0;)
      f32.const 0x1p+0 (;=1;)
      f32.const 0x0p+0 (;=0;)
      call 8
      local.set 2
      local.get 0
      local.get 0
      i32.load
      local.get 2
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 1376
    i32.store
    local.get 1
    i32.load
    i32.const 1376
    call 0
    if  ;; label = @1
      f32.const 0x0p+0 (;=0;)
      f32.const -0x1p+0 (;=-1;)
      f32.const 0x0p+0 (;=0;)
      call 8
      local.set 2
      local.get 0
      local.get 0
      i32.load
      local.get 2
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 1440
    i32.store
    local.get 1
    i32.load
    i32.const 1440
    call 0
    if  ;; label = @1
      f32.const 0x0p+0 (;=0;)
      f32.const 0x0p+0 (;=0;)
      f32.const -0x1p+0 (;=-1;)
      call 8
      local.set 2
      local.get 0
      local.get 0
      i32.load
      local.get 2
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 1504
    i32.store
    local.get 1
    i32.load
    i32.const 1504
    call 0
    if  ;; label = @1
      f32.const 0x0p+0 (;=0;)
      f32.const 0x0p+0 (;=0;)
      f32.const 0x1p+0 (;=1;)
      call 8
      local.set 1
      local.get 0
      local.get 0
      i32.load
      local.get 1
      i32.load
      call 3
      i32.store
    end
    global.get 1
    i32.const 4
    i32.add
    global.set 1)
  (memory (;0;) 1)
  (global (;0;) (mut i32) (i32.const 0))
  (global (;1;) (mut i32) (i32.const 17932))
  (export "run" (func 9))
  (export "memory" (memory 0))
  (data (;0;) (i32.const 1036) "<")
  (data (;1;) (i32.const 1048) "\01\00\00\00\22\00\00\00p\00l\00a\00y\00e\00r\00_\00l\00e\00f\00t\00_\00s\00t\00a\00r\00t")
  (data (;2;) (i32.const 1100) "<")
  (data (;3;) (i32.const 1112) "\01\00\00\00\1e\00\00\00~\00l\00i\00b\00/\00r\00t\00/\00t\00l\00s\00f\00.\00t\00s")
  (data (;4;) (i32.const 1164) "<")
  (data (;5;) (i32.const 1176) "\01\00\00\00(\00\00\00A\00l\00l\00o\00c\00a\00t\00i\00o\00n\00 \00t\00o\00o\00 \00l\00a\00r\00g\00e")
  (data (;6;) (i32.const 1228) "<")
  (data (;7;) (i32.const 1240) "\01\00\00\00$\00\00\00p\00l\00a\00y\00e\00r\00_\00r\00i\00g\00h\00t\00_\00s\00t\00a\00r\00t")
  (data (;8;) (i32.const 1292) "<")
  (data (;9;) (i32.const 1304) "\01\00\00\00\1e\00\00\00p\00l\00a\00y\00e\00r\00_\00u\00p\00_\00s\00t\00a\00r\00t")
  (data (;10;) (i32.const 1356) "<")
  (data (;11;) (i32.const 1368) "\01\00\00\00\22\00\00\00p\00l\00a\00y\00e\00r\00_\00d\00o\00w\00n\00_\00s\00t\00a\00r\00t")
  (data (;12;) (i32.const 1420) "<")
  (data (;13;) (i32.const 1432) "\01\00\00\00(\00\00\00p\00l\00a\00y\00e\00r\00_\00f\00o\00r\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t")
  (data (;14;) (i32.const 1484) "<")
  (data (;15;) (i32.const 1496) "\01\00\00\00*\00\00\00p\00l\00a\00y\00e\00r\00_\00b\00a\00c\00k\00w\00a\00r\00d\00_\00s\00t\00a\00r\00t"))
