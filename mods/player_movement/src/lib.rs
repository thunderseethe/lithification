//! This is very unidiomatic rust code but is written as such to see what kind of wasm will get generated.
//! It has the advantage of generating tighter code. If we don't mark our functions as `extern "C"` we do a lot of stack management at function boundaries that isn't necessary for wasm
//! Similarly if we don't use `static mut`s to carry data across functions we heap alloc and other uneccesary complexities

//TODO: Fix this to be less hardcoded?
wit_bindgen_rust::import!("../math/math.wit");
unsafe impl bytemuck::Zeroable for math::GlobalTransform {}
unsafe impl bytemuck::Pod for math::GlobalTransform {}

wit_bindgen_rust::import!("../../interface/event.wit");

// TODO: This should be wrapped up as it's own crate for API stability and reuse
wit_bindgen_rust::import!("../../interface/resources.wit");
impl<'a, T: bytemuck::NoUninit> From<&'a T> for resources::ResourceParam<'a> {
    fn from(bytes_to_be: &'a T) -> Self {
        resources::ResourceParam { repr: bytemuck::bytes_of(bytes_to_be) }
    }
}
fn init_resource<'a, T: bytemuck::NoUninit>(name: &str, resource: &'a T) -> resources::ResourceId {
    resources::init_resource(name, resources::ResourceParam::from(resource))
}
fn get_resource<T: bytemuck::Pod>(id: resources::ResourceId) -> T {
    let resource = resources::get_resource(id);
    bytemuck::pod_read_unaligned(&resource.repr[..])
}
fn write_resource<'a, T: bytemuck::NoUninit>(id: resources::ResourceId, resource: &'a T) {
    resources::write_resource(id, resources::ResourceParam::from(resource));
}

static mut PLAYER_LEFT_START: event::EventId = event::EventId { id: 0 };
static mut PLAYER_RIGHT_START: event::EventId = event::EventId { id: 0 };
static mut PLAYER_UP_START: event::EventId = event::EventId { id: 0 };
static mut PLAYER_DOWN_START: event::EventId = event::EventId { id: 0 };
static mut PLAYER_FORWARD_START: event::EventId = event::EventId { id: 0 };
static mut PLAYER_BACKWARD_START: event::EventId = event::EventId { id: 0 };

static mut GLOBAL_TRANSFORM_RES: resources::ResourceId = resources::ResourceId { id: 0 };

#[no_mangle]
pub unsafe extern "C" fn init() {
    PLAYER_LEFT_START = event::new_event_id("player_left_start");
    PLAYER_RIGHT_START = event::new_event_id("player_right_start");
    PLAYER_UP_START = event::new_event_id("player_up_start");
    PLAYER_DOWN_START = event::new_event_id("player_down_start");
    PLAYER_FORWARD_START = event::new_event_id("player_forward_start");
    PLAYER_BACKWARD_START = event::new_event_id("player_backward_start");

    let id_transform = math::identity_global_transform();
    let trans_bytes = bytemuck::bytes_of(&id_transform);
    GLOBAL_TRANSFORM_RES = resources::init_resource("global_transform", resources::ResourceParam { repr: trans_bytes });
}

macro_rules! event_case {
    ($msg:ident, $event0:expr => $body0:tt, $($event:expr => $body:tt,)* _ => $body_default:expr) => {
        if event::event_id_eq($msg.tag, unsafe {$event0}) {
            $body0
        }
        $(else if event::event_id_eq($msg.tag, unsafe{$event}) {
            $body
        })*
        else {
            $body_default
        }
    };
    ($msg:ident, $event0:expr => $body0:tt, $($event:expr => $body:tt),*) => {
        event_case!($msg, $event0 => $body0, $($event => $body,)* _ => {})
    } 
}

// TODO: it would be nice to wrap this up as a macro to remove a lot of this boilerplate
#[no_mangle]
pub extern "C" fn run(msg: Box<event::Message>) {
    let mut transform: math::GlobalTransform = get_resource(unsafe { GLOBAL_TRANSFORM_RES });

    event_case!(msg, 
        PLAYER_LEFT_START => {
            transform = math::append_translation(transform, math::x_axis_vec3())
        },
        PLAYER_RIGHT_START => {
            transform = math::append_translation(transform, math::new_vec3(-1.0, 0.0, 0.0))
        },
        PLAYER_UP_START => {
            transform = math::append_translation(transform, math::y_axis_vec3())
        },
        PLAYER_DOWN_START => {
            transform = math::append_translation(transform, math::new_vec3(0.0, -1.0, 0.0))
        },
        PLAYER_BACKWARD_START => {
            transform = math::append_translation(transform, math::z_axis_vec3())
        },
        PLAYER_FORWARD_START => {
            transform = math::append_translation(transform, math::new_vec3(0.0, 0.0, -1.0))
        });

    write_resource(unsafe { GLOBAL_TRANSFORM_RES }, &transform);
}
