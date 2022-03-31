// The entry file of your WebAssembly module.
import { GlobalTransform, Vector3 } from "./math";
import { Message } from "./common";

export function run(transform: GlobalTransform, msg: Message): void {
  if (msg.is_event("player_left_start")) {
    transform.append_translation(new Vector3(-1.0, 0.0, 0.0));
  }
  if (msg.is_event("player_right_start")) {
    transform.append_translation(new Vector3(1.0, 0.0, 0.0));
  }
  if (msg.is_event("player_up_start")) {
    transform.append_translation(new Vector3(0.0, 1.0, 0.0));
  }
  if (msg.is_event("player_down_start")) {
    transform.append_translation(new Vector3(0.0, -1.0, 0.0));
  }
  if (msg.is_event("player_forward_start")) {
    transform.append_translation(new Vector3(0.0, 0.0, -1.0));
  }
  if (msg.is_event("player_backward_start")) {
    transform.append_translation(new Vector3(0.0, 0.0, 1.0));
  }
}