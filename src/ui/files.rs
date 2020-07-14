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

#[derive(Debug, Clone)]
pub enum Message {
    Ignore,
    DirClicked(usize),
    FileClicked(usize),
    Input(String),
    Open(PathBuf),
    Save(PathBuf),
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
            folder_svg: icon::FOLDER.svg().height(Length::Units(15)),
            file_svg: icon::FILE.svg().height(Length::Units(15)),
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
    scroll: scrollable::State,
    input: text_input::State,
    text: String
}

impl FSMenu {
    fn new<T: AsRef<Path>>(path: T) -> Self {
        Self {
            path: PathBuf::from(path.as_ref()),
            list: Files::new(path.as_ref()),
            selected: None,
            scroll: Default::default(),
            input: Default::default(),
            text: Default::default(),
        }
    }
    pub fn open<T: AsRef<Path>>(path: T) -> impl AppWidget<Message=Message> {
        OpenDialog {
            btn_completed: Default::default(),
            fs_menu: Self::new(path),
        }

    }
    pub fn save<T: AsRef<Path>>(path: T) -> impl AppWidget<Message=Message> {
        SaveDialog {
            btn_completed: Default::default(),
            fs_menu: Self::new(path),
        }
    }

    pub fn selected(&self) -> Option<PathBuf> {
        Some(match &self.selected {
            None => {
                let mut path = self.path.clone();
                path.push(self.text.as_str());
                path
            },
            Some(path) => {path.clone()},
        })
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
                self.text.clear();
                self.scroll = Default::default();
            },
            Message::FileClicked(n) => {
                let name = list.file_name(n)
                    .ok_or(io::Error::new(ErrorKind::InvalidInput, "selected file not found"))?;
                let mut selected = self.path.clone();
                selected.push(name);
                let mut text = &mut self.text;
                text.clear();
                text.push_str(name.to_string_lossy().as_ref());
                self.selected = Some(selected);
            },
            Message::Input(text) => {
                self.selected = None;
                self.text = text;
            },
            Message::Open(_) => {/*need to process in caller*/},
            Message::Save(_) => {/*need to process in caller*/},
            Message::Ignore => {},
        };
        Ok(())
    }

    fn view_with_btn<'a>(&'a mut self, btn: Button<'a, Message>) -> Element<'a, Message> {
        let text = self.text.clone();
        match &mut self.list {
            Ok(list) => {
                Column::new()
                    .push(Scrollable::new(&mut self.scroll).height(Length::Fill).push(list.view()))
                    .push(Container::new(
                        Row::new()
                            .push(TextInput::new(
                                &mut self.input,
                                &"",
                                text.as_str(),
                                |s| Message::Input(s)
                            ).size(15).width(Length::Units(150)))
                            .push(btn)
                            .align_items(Align::Center)
                    ).height(Length::Units(30)))
                    .into()
            },
            Err(e) => { Text::new(e.to_string()).into() },
        }
    }

    fn update(&mut self, msg: Message) {
        self.update_with_err(msg);
    }
}

pub struct OpenDialog {
    btn_completed: button::State,
    fs_menu: FSMenu,
}


impl AppWidget for OpenDialog {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut btn = Button::new(&mut self.btn_completed, icon::OPEN.svg());
        if let Some(selected) = self.fs_menu.selected() {
            btn = btn.on_press(Message::Open(selected));
        }
        self.fs_menu.view_with_btn(btn)
    }

    fn update(&mut self, msg: Self::Message) {
        self.fs_menu.update(msg)
    }
}


pub struct SaveDialog {
    btn_completed: button::State,
    fs_menu: FSMenu,
}


impl AppWidget for SaveDialog {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Self::Message> {
        let mut btn = Button::new(&mut self.btn_completed, icon::SAVE.svg());
        if let Some(selected) = self.fs_menu.selected() {
            btn = btn.on_press(Message::Save(selected));
        }
        self.fs_menu.view_with_btn(btn)
    }

    fn update(&mut self, msg: Self::Message) {
        self.fs_menu.update(msg)
    }
}