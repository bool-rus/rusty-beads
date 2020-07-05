pub const BEADS_LINE: SvgData = SvgData(include_bytes!("../../resources/beads-line-icon.svg"));

pub const ADD_LEFT_COLUMN: SvgData = SvgData(include_bytes!("../../resources/add-left-column.svg"));
pub const ADD_RIGHT_COLUMN: SvgData = SvgData(include_bytes!("../../resources/add-right-column.svg"));
pub const ADD_TOP_ROW: SvgData = SvgData(include_bytes!("../../resources/add-top-row.svg"));
pub const ADD_BOTTOM_ROW: SvgData = SvgData(include_bytes!("../../resources/add-bottom-row.svg"));

pub const REMOVE_LEFT_COLUMN: SvgData = SvgData(include_bytes!("../../resources/remove-left-column.svg"));
pub const REMOVE_RIGHT_COLUMN: SvgData = SvgData(include_bytes!("../../resources/remove-right-column.svg"));
pub const REMOVE_TOP_ROW: SvgData = SvgData(include_bytes!("../../resources/remove-top-row.svg"));
pub const REMOVE_BOTTOM_ROW: SvgData = SvgData(include_bytes!("../../resources/remove-bottom-row.svg"));

pub const UNDO: SvgData = SvgData(include_bytes!("../../resources/undo.svg"));
pub const REDO: SvgData = SvgData(include_bytes!("../../resources/redo.svg"));

pub const ZOOM_IN: SvgData = SvgData(include_bytes!("../../resources/zoom-in.svg"));
pub const ZOOM_OUT: SvgData = SvgData(include_bytes!("../../resources/zoom-out.svg"));

pub const RESIZE: SvgData = SvgData(include_bytes!("../../resources/resize.svg"));

pub const CHANGE_SCHEMA: SvgData = SvgData(include_bytes!("../../resources/change-schema.svg"));

pub const SAVE: SvgData = SvgData(include_bytes!("../../resources/save.svg"));
pub const OPEN: SvgData = SvgData(include_bytes!("../../resources/open.svg"));
pub const FOLDER: SvgData = SvgData(include_bytes!("../../resources/folder.svg"));
pub const FILE: SvgData = SvgData(include_bytes!("../../resources/file.svg"));

use iced::{Svg, svg};
pub struct SvgData(&'static [u8]);

impl SvgData {
    pub fn svg(&self) -> Svg {
        Svg::new(svg::Handle::from_memory(self.0))
    }
}
