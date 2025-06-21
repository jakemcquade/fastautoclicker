export function buildPayout(name: string, value: string | number) {
  if (name === "mouse_button" || name === "click_type") {
    return { U8: value };
  }

  if (typeof value === "number") {
    return { U64: value };
  }

  return { String: value };
}
