(module
  (import "env" "memory" (memory 1))
  (import "env" "log" (func $log (param i32)))
  (import "env" "log_char" (func $log_char (param i32)))
  (global $droplet_count (mut i32) (i32.const 0))
  (global $max_droplets (i32) (i32.const 100))
  (global $stack_ptr (mut i32) (i32.const 1024))
  (global $stack_base (i32) (i32.const 1024))
  (global $call_stack_ptr (mut i32) (i32.const 2048))
  (global $call_stack_base (i32) (i32.const 2048))
  (global $droplets_base (i32) (i32.const 4096))
  (global $reservoir_base (i32) (i32.const 8192))
  (func $get_grid_char (param $x i32) (param $y i32) (result i32)
    ;; This would need to access a memory representation of the grid
    ;; For now, using a data segment to store the grid
    local.get $y
    i32.const 5
    i32.mul
    local.get $x
    i32.add
    ;; This would access the grid in memory
    ;; Placeholder - return 0 for now
    i32.const 0
  )
  (func $stack_push (param $value i32)
    global.get $stack_ptr
    local.get $value
    i32.store
    global.get $stack_ptr
    i32.const 4
    i32.add
    global.set $stack_ptr
  )
  (func $stack_pop (result i32)
    global.get $stack_ptr
    i32.const 4
    i32.sub
    global.tee $stack_ptr
    i32.load
  )
  (func $reservoir_get (param $x i32) (param $y i32) (result i32)
    ;; Map 2D (x,y) to 1D index: y*width + x, then multiply by 4 for i32 size
    local.get $y
    i32.const 5
    i32.mul
    local.get $x
    i32.add
    i32.const 4
    i32.mul
    global.get $reservoir_base
    i32.add
    i32.load
  )
  (func $reservoir_set (param $x i32) (param $y i32) (param $value i32)
    ;; Map 2D (x,y) to 1D index: y*width + x, then multiply by 4 for i32 size
    local.get $y
    i32.const 5
    i32.mul
    local.get $x
    i32.add
    i32.const 4
    i32.mul
    global.get $reservoir_base
    i32.add
    local.get $value
    i32.store
  )
  (func $create_droplet (param $x i32) (param $y i32) (param $dir_idx i32) (param $value i32)
    ;; Check if we can create a new droplet
    global.get $droplet_count
    global.get $max_droplets
    i32.ge_s
    if
      return
    end
    ;; Calculate position in droplet array
    global.get $droplet_count
    i32.const 4
    i32.mul
    global.get $droplets_base
    i32.add
    ;; Store x coordinate
    local.get $x
    i32.store
    ;; Store y coordinate
    global.get $droplet_count
    i32.const 4
    i32.mul
    global.get $droplets_base
    i32.add
    i32.const 4
    i32.add
    local.get $y
    i32.store
    ;; Store direction
    global.get $droplet_count
    i32.const 4
    i32.mul
    global.get $droplets_base
    i32.add
    i32.const 8
    i32.add
    local.get $dir_idx
    i32.store
    ;; Store value
    global.get $droplet_count
    i32.const 4
    i32.mul
    global.get $droplets_base
    i32.add
    i32.const 12
    i32.add
    local.get $value
    i32.store
    ;; Increment droplet count
    global.get $droplet_count
    i32.const 1
    i32.add
    global.set $droplet_count
  )
  (func $op_push (param $current_value i32)
    call $stack_push
    ;; Value is still available as a return value if needed
    local.get $current_value
    drop
  )
  (func $op_number_1 (result i32)
    i32.const 1
  )
  (func $op_pop (result i32)
    call $stack_pop
  )
  (func $op_number_0 (result i32)
    i32.const 0
  )
  (func $op_increment (param $current_value i32) (result i32)
    local.get $current_value
    i32.const 1
    i32.add
  )
  (func $execute (export "execute")
    ;; Create initial droplet at start position
    i32.const 0
    i32.const 0
    i32.const 1
    i32.const 0
    call $create_droplet
    ;; Main simulation loop (max 10000 ticks)
    (local $tick_count i32)
    (local $i i32)
    (local $current_x i32)
    (local $current_y i32)
    (local $current_dir i32)
    (local $current_value i32)
    (local $new_x i32)
    (local $new_y i32)
    (local $char_code i32)
    (local $result_value i32)
    
    (loop $main_loop
      ;; Check termination conditions
      local.get $tick_count
      i32.const 10000
      i32.ge_s
      if
        ;; Max ticks reached
        br $main_loop_end
      end
      global.get $droplet_count
      i32.eqz
      if
        ;; No more droplets
        br $main_loop_end
      end
      
      ;; Process all current droplets for this tick
      local.set $i (i32.const 0)
      (loop $process_droplets
        ;; Check if we've processed all droplets
        local.get $i
        global.get $droplet_count
        i32.ge_s
        br_if $process_droplets_end
        
        ;; Get the current droplet data
        ;; Calculate memory address for droplet i
        local.get $i
        i32.const 4
        i32.mul
        global.get $droplets_base
        i32.add
        ;; Load x
        i32.load
        local.set $current_x
        ;; Load y
        local.get $i
        i32.const 4
        i32.mul
        global.get $droplets_base
        i32.add
        i32.const 4
        i32.add
        i32.load
        local.set $current_y
        ;; Load direction
        local.get $i
        i32.const 4
        i32.mul
        global.get $droplets_base
        i32.add
        i32.const 8
        i32.add
        i32.load
        local.set $current_dir
        ;; Load value
        local.get $i
        i32.const 4
        i32.mul
        global.get $droplets_base
        i32.add
        i32.const 12
        i32.add
        i32.load
        local.set $current_value
        
        ;; Get character at droplet position
        local.get $current_x
        local.get $current_y
        call $get_grid_char
        local.set $char_code
        
        ;; Process the character
        ;; In a real implementation, we would have a large if/else chain
        ;; or use a br_table to jump to the appropriate handler
        ;; For simplicity in this example, we'll handle a few key cases
        local.get $char_code
        i32.const 48
        i32.sub
        local.tee $result_value
        i32.const 0
        i32.ge_s
        local.get $result_value
        i32.const 9
        i32.le_s
        i32.and
        if
          ;; Create new droplet with digit value, direction DOWN
          local.get $current_x
          local.get $current_y
          i32.const 1
          local.get $result_value
          call $create_droplet
          ;; Remove current droplet by marking it for deletion
          ;; (In a real implementation, we would handle this properly)
        else
          ;; Handle other characters - this is a simplified version
          ;; In reality, each character would have its own handling logic
          block $skip_char_handling
            ;; If char is '!' (output), handle it
            local.get $char_code
            i32.const 33
            i32.eq
            if
              local.get $current_value
              call $log
              ;; Remove this droplet (output sink)
              br $skip_char_handling
            end
            ;; If char is '+' (increment), handle it
            local.get $char_code
            i32.const 43
            i32.eq
            if
              local.get $current_value
              call $op_increment
              local.set $current_value
              ;; Update the droplet's value in memory
              local.get $i
              i32.const 4
              i32.mul
              global.get $droplets_base
              i32.add
              i32.const 12
              i32.add
              local.get $current_value
              i32.store
              br $skip_char_handling
            end
            ;; Add more character handling as needed
            ;; For now, just continue moving in the current direction
            local.get $current_x
            local.set $new_x
            local.get $current_y
            local.set $new_y
            local.get $current_dir
            i32.const 0
            i32.eq
            if
              local.get $current_y
              i32.const 1
              i32.sub
              local.set $new_y
            end
            local.get $current_dir
            i32.const 1
            i32.eq
            if
              local.get $current_y
              i32.const 1
              i32.add
              local.set $new_y
            end
            local.get $current_dir
            i32.const 2
            i32.eq
            if
              local.get $current_x
              i32.const 1
              i32.sub
              local.set $new_x
            end
            local.get $current_dir
            i32.const 3
            i32.eq
            if
              local.get $current_x
              i32.const 1
              i32.add
              local.set $new_x
            end
            ;; Update the droplet's position in memory
            local.get $i
            i32.const 4
            i32.mul
            global.get $droplets_base
            i32.add
            local.get $new_x
            i32.store
            local.get $i
            i32.const 4
            i32.mul
            global.get $droplets_base
            i32.add
            i32.const 4
            i32.add
            local.get $new_y
            i32.store
          end
        end
        
        local.get $i
        i32.const 1
        i32.add
        local.set $i
        br $process_droplets
      end
      
      ;; Increment tick counter
      local.get $tick_count
      i32.const 1
      i32.add
      local.set $tick_count
      br $main_loop
    end
    
    ;; Label to break to when ending the loop
    block $main_loop_end
    end
  )
)