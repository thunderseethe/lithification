export declare function is_event(ptr: usize, event: String): bool;

@unmanaged
export class Message {
ptr: usize; 

is_event(event: String): bool {
    return is_event(this.ptr, event);
}
}