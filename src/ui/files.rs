use crate::reimport::*;
use super::AppWidget;
use iced::{Element, Svg, svg};
use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};
use std::io;
use std::io::{Error, ErrorKind};
use crate::ui::style::FSMenuItem;
use crate::ui::icon;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Ignore,
    DirClicked(usize),
    FileClicked(usize),
    Open,
    Save,
}

struct Files {
    folder_svg: Svg,
    file_svg: Svg,
    dirs: Vec<(button::State, OsString)>,
    files: Vec<(button::State, OsString)>,
}

impl Files {
    fn new<T: AsRef<Path>>(path: T) -> io::Result<Self> {
        let mut dirs = vec!["..".into()];
        let mut files = Vec::new();
        let path = path.as_ref();
        for entry in path.read_dir()? {
            let entry = entry?.path();
            if entry.is_dir() {
                entry.file_name().map(|name|dirs.push(name.to_owned()));
            }
            if entry.is_file() {
                entry.file_name().map(|name|files.push(name.to_owned()));
            }
        }
        dirs.sort_unstable();
        files.sort_unstable();
        Ok(Self {
            folder_svg: Svg::new(svg::Handle::from_memory(icon::FOLDER)).height(Length::Units(15)),
            file_svg: Svg::new(svg::Handle::from_memory(icon::FILE)).height(Length::Units(15)),
            dirs: dirs.into_iter().map(|name|(Default::default(), name)).collect(),
            files: files.into_iter().map(|name|(Default::default(), name)).collect(),
        })
    }

    fn dir_name(&self, n: usize) -> Option<&OsString> {
        self.dirs.get(n).map(|(_, name)|name)
    }

    fn file_name(&self, n: usize) -> Option<&OsString> {
        self.files.get(n).map(|(_, name)|name)
    }

    fn view(&mut self) -> Column<'_, Message> {
        let folder_icon = self.folder_svg.clone();
        let file_icon = self.file_svg.clone();
        let dirs = self.dirs.iter_mut().enumerate()
            .map(|(i, (state, name))| {
                Button::new(
                    state,
                    Row::new()
                        .push(folder_icon.clone())
                        .push(Text::new(name.to_string_lossy()).size(15))
                ).on_press(Message::DirClicked(i)).style(FSMenuItem).into()
            });
        let files = self.files.iter_mut().enumerate()
            .map(|(i, (state, name))| {
                Button::new(
                    state,
                    Row::new()
                        .push(file_icon.clone())
                        .push(Text::new(name.to_string_lossy()).size(15))
                ).on_press(Message::FileClicked(i)).style(FSMenuItem).into()
            });
        Column::with_children(dirs.chain(files).collect())
    }
}


pub struct FSMenu {
    path: PathBuf,
    list: io::Result<Files>,
    selected: Option<PathBuf>,
    btn_completed: button::State,
    scroll: scrollable::State,
    submit_icon: Svg,
    submit_msg: Message,
    input: text_input::State,
    text: Rc<RefCell<String>>
}

impl FSMenu {
    pub fn open<T: AsRef<Path>>(path: T) -> Self {
        let list = Files::new(path.as_ref());
        Self {
            path: PathBuf::from(path.as_ref()),
            list,
            selected: None,
            btn_completed: Default::default(),
            scroll: Default::default(),
            submit_icon: Svg::new(svg::Handle::from_memory(icon::OPEN)),
            input: Default::default(),
            text: Rc::new(RefCell::new(String::new())),
            submit_msg: Message::Open,
        }
    }
    pub fn save<T: AsRef<Path>>(path: T) -> Self {
        let mut obj = Self::open(path);
        obj.submit_icon = Svg::new(svg::Handle::from_memory(icon::SAVE));
        obj.submit_msg = Message::Save;
        obj
    }
    fn update_with_err(&mut self, msg: Message) -> io::Result<()> {
        let list = self.list.as_ref().map_err(|e|{
            io::Error::new(e.kind(), "File list is not constructed")
        })?;
        match msg {
            Message::DirClicked(n) => {
                let name = list.dir_name(n)
                    .ok_or(io::Error::new(ErrorKind::InvalidInput, "selected folder not found"))?;
                self.path.push(name);
                self.path = self.path.canonicalize()?;
                self.list = Files::new(&self.path);
                self.selected = None;
                self.scroll = Default::default();
            },
            Message::FileClicked(n) => {
                let name = list.file_name(n)
                    .ok_or(io::Error::new(ErrorKind::InvalidInput, "selected file not found"))?;
                let mut selected = self.path.clone();
                selected.push(name);
                self.selected = Some(selected);
            },
            Message::Open => {/*need to process in caller*/},
            Message::Save => {/*need to process in caller*/},
            Message::Ignore => {},
        };
        Ok(())
    }
}

impl AppWidget for FSMenu {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Self::Message> {
        let text = self.text.clone();
        match &mut self.list {
            Ok(list) => {
                let mut btn = Button::new(&mut self.btn_completed, self.submit_icon.clone());
                if self.selected.is_some() {
                    btn = btn.on_press(self.submit_msg);
                }
                Column::new()
                    .push(Scrollable::new(&mut self.scroll).height(Length::Fill).push(list.view()))
                    .push(Container::new(
                        Row::new()
                            .push(TextInput::new(
                                &mut self.input,
                                &"",
                                text.clone().borrow().as_str(),
                                move |s|{
                                    let mut x = text.borrow_mut();
                                    x.clear();
                                    x.push_str(s.as_str());
                                    Message::Ignore
                                }
                            ).size(15).width(Length::Units(150)))
                            .push(btn)
                            .align_items(Align::Center)
                    ).height(Length::Units(30)))
                    .into()
            },
            Err(e) => { Text::new(e.to_string()).into() },
        }
    }

    fn update(&mut self, msg: Self::Message) {
        self.update_with_err(msg);
    }
}