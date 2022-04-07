export declare function intern_string(str: usize, len: usize): u32;

import { Sized } from "./math";

@unmanaged
export class Symbol {
    token: u32;

    constructor(s: String) {
        this.token = intern_string(changetype<usize>(s), s.length);
    }
}

export declare const MESSAGE_SIZE: usize;
export declare function is_event(ptr: usize, event: u32): bool;
export declare function message_empty(self: usize): void;

@unmanaged
export class Message extends Sized {
    tag: u32;
    payload: ArrayBuffer;

size(): usize {
  return MESSAGE_SIZE;
}

is_event(event: Symbol): bool {
    return is_event(changetype<usize>(this), event.token);
}

static empty(): Message {
    let ptr = heap.alloc(MESSAGE_SIZE);
    message_empty(ptr);
    return changetype<Message>(ptr);
}
}

export declare function register_resource_inner(ptr: usize, size: usize): void;
export function register_resource<T extends Sized>(res: T): void {
    register_resource_inner(changetype<usize>(res), res.size());
}