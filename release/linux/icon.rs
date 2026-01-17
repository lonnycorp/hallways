use crate::icon::{self, IconSpec};

pub const ICON_SPECS: &[IconSpec] = &[
    IconSpec::Icon16,
    IconSpec::Icon16At2x,
    IconSpec::Icon32,
    IconSpec::Icon32At2x,
    IconSpec::Icon128,
    IconSpec::Icon128At2x,
    IconSpec::Icon256,
];

pub fn linux_iconset_render() {
    icon::iconset_render(ICON_SPECS);
}
