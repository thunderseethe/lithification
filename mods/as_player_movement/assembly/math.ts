export declare const GLOBAL_TRANSFORM_SIZE: usize;
export declare function new_global_transform(self: usize, pos: usize, rot: usize, scale: usize): void;
export declare function global_transform_identity(self: usize): void;
export declare function append_translation(self: usize, translation: usize): usize;
export declare function prepend_translation(self: usize, translation: usize): usize;

@unmanaged
export abstract class Sized {
    abstract size(): usize;
}

@unmanaged
export class GlobalTransform extends Sized {

size(): usize { return GLOBAL_TRANSFORM_SIZE }

static new(position: Vec3, rotation: Quat, scale: Vec3): GlobalTransform {
    let ptr = heap.alloc(GLOBAL_TRANSFORM_SIZE);
    new_global_transform(ptr, changetype<usize>(position), changetype<usize>(rotation), changetype<usize>(scale));
    return changetype<GlobalTransform>(ptr);
}

static identity(): GlobalTransform {
    let ptr = heap.alloc(GLOBAL_TRANSFORM_SIZE);
    global_transform_identity(ptr);
    return changetype<GlobalTransform>(ptr);
}

append_translation(translation: Vec3): GlobalTransform {
    append_translation(changetype<usize>(this), changetype<usize>(translation));
    return this;
}

prepend_translation(translation: Vec3): GlobalTransform {
    prepend_translation(changetype<usize>(this), changetype<usize>(translation));
    return this;
}

}

export declare function new_vec3(self: usize, x: f32, y: f32, z: f32): usize;
export declare function vec3_x_axis(self: usize): void;
export declare function vec3_y_axis(self: usize): void;
export declare function vec3_z_axis(self: usize): void;

@unmanaged
export class Vec3 extends Sized {

//these fields exist purely for sizing
private _x: f32;
private _y: f32;
private _z: f32;

size(): usize { return sizeof<Vec3>() }

constructor(x: f32, y: f32, z: f32) {
    super();
    new_vec3(changetype<usize>(this), x, y, z);
}

static x_axis(): Vec3 {
    let ptr = memory.data(sizeof<Vec3>());
    vec3_x_axis(ptr);
    return changetype<Vec3>(ptr);
}

static y_axis(): Vec3 {
    let ptr = memory.data(sizeof<Vec3>());
    vec3_y_axis(ptr);
    return changetype<Vec3>(ptr);
}

static z_axis(): Vec3 {
    let ptr = memory.data(sizeof<Vec3>());
    vec3_z_axis(ptr);
    return changetype<Vec3>(ptr);
}

}


export declare function quat_id(self: usize): void;

@unmanaged
export class Quat extends Sized {
    private _x: f32;
    private _y: f32;
    private _z: f32;
    private _w: f32;

    size(): usize { return sizeof<Quat>(); }

    /* come back to this */
    static identity(): Quat {
        let ptr = memory.data(sizeof<Quat>());
        quat_id(ptr);
        return changetype<Quat>(ptr);
    }
}