// The entry file of your WebAssembly module.

@unmanaged
export class Position {
  x: i32
  y: i32
  z: i32
}

export function register_event(): void {

}

export function run(pos: Position): Position {
  pos.y += 1;
  return pos;
}