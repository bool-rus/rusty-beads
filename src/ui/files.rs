use crate::reimport::*;
use super::AppWidget;
use iced::Element;
use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};
use std::io;
use std::io::{Error, ErrorKind};
use crate::ui::style::FSMenuItem;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    DirClicked(usize),
    FileClicked(usize),
    Completed,
}

struct Files {
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
        let dirs = self.dirs.iter_mut().enumerate()
            .map(|(i, (state, name))| {
                Button::new(
                    state,
                    Row::new()
                        .push(Text::new("DIR|").size(15))
                        .push(Text::new(name.to_string_lossy()).size(15))
                ).on_press(Message::DirClicked(i)).style(FSMenuItem).into()
            });
        let files = self.files.iter_mut().enumerate()
            .map(|(i, (state, name))| {
                Button::new(
                    state,
                    Row::new()
                        .push(Text::new("FILE|").size(15))
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
}

impl FSMenu {
    pub fn new<T: AsRef<Path>>(path: T) -> Self {
        let list = Files::new(path.as_ref());
        Self {
            path: PathBuf::from(path.as_ref()),
            list,
            selected: None,
            btn_completed: Default::default(),
            scroll: Default::default(),
        }
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
            Message::Completed => {/*need to process in caller*/},
        };
        Ok(())
    }
}

impl AppWidget for FSMenu {
    type Message = Message;

    fn view(&mut self) -> Element<'_, Self::Message> {
        match &mut self.list {
            Ok(list) => {
                let mut btn = Button::new(&mut self.btn_completed, Text::new("OK"))
                    ;
                if self.selected.is_some() {
                    btn = btn.on_press(Message::Completed);
                }
                Column::new()
                    .push(Scrollable::new(&mut self.scroll).height(Length::Fill).push(list.view()))
                    .push(Container::new(btn).height(Length::Units(60)))
                    .into()
            },
            Err(e) => { Text::new(e.to_string()).into() },
        }
    }

    fn update(&mut self, msg: Self::Message) {
        self.update_with_err(msg);
    }
}