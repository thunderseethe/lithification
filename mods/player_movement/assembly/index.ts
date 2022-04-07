// The entry file of your WebAssembly module.
import { GlobalTransform, Vec3 } from "./math";
import { Message, register_resource, Symbol } from "./host";

const PLAYER_LEFT_START = new Symbol("player_left_start");
const PLAYER_RIGHT_START = new Symbol("player_right_start");
const PLAYER_UP_START = new Symbol("player_up_start");
const PLAYER_DOWN_START = new Symbol("player_down_start");
const PLAYER_FORWARD_START = new Symbol("player_forward_start");
const PLAYER_BACKWARD_START = new Symbol("player_backward_start");

export function init(): void {
  register_resource<GlobalTransform>(GlobalTransform.identity());
  register_resource<Message>(Message.empty());
}

export function run(transform: GlobalTransform, msg: Message): void {
  if (msg.is_event(PLAYER_LEFT_START)) {
    transform.append_translation(new Vec3(-1.0, 0.0, 0.0));
  }
  if (msg.is_event(PLAYER_RIGHT_START)) {
    transform.append_translation(Vec3.x_axis());
  }
  if (msg.is_event(PLAYER_UP_START)) {
    transform.append_translation(Vec3.y_axis());
  }
  if (msg.is_event(PLAYER_DOWN_START)) {
    transform.append_translation(new Vec3(0.0, -1.0, 0.0));
  }
  if (msg.is_event(PLAYER_FORWARD_START)) {
    transform.append_translation(new Vec3(0.0, 0.0, -1.0));
  }
  if (msg.is_event(PLAYER_BACKWARD_START)) {
    transform.append_translation(Vec3.z_axis());
  }
}