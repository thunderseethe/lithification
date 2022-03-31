export declare function append_translation(self: usize, translation: usize): usize;
export declare function prepend_translation(self: usize, translation: usize): usize;
export declare function new_vec3_f32(x: f32, y: f32, z: f32): usize;

@unmanaged
export class GlobalTransform {
ptr: usize;

append_translation(translation: Vector3): GlobalTransform {
    this.ptr = append_translation(this.ptr, translation.ptr);
    return this;
}

prepend_translation(translation: Vector3): GlobalTransform {
    this.ptr = prepend_translation(this.ptr, translation.ptr);
    return this;
}
}

@unmanaged
export class Vector3 {
ptr: usize;

constructor(x: f32, y: f32, z: f32) {
    this.ptr = new_vec3_f32(x, y, z);
}
}