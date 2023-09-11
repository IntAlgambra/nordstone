use std::collections::HashMap;
use iced::{Application, Command, Element, Renderer, Settings, Theme};
use iced::widget::{button, row, text, text_input, column, Column};
use home::home_dir;
use iced::futures::StreamExt;

mod models;
mod encryption;
mod storage;

use models::Folder;
use encryption::AgeEncryptor;
use storage::LocalStorageManager;
use crate::models::Record;
use crate::storage::StorageManager;

#[derive(Debug)]
struct NordstoneUi {
    state: MainState,
    subfolder_to_edit: Option<usize>,
    record_to_edit: Option<usize>,
    key: Option<String>,
    records: Vec<RecordUi>,
}

impl NordstoneUi {
    fn decrypt(&mut self, key: String) {
        self.key = Some(key.clone());
        let encryptor = AgeEncryptor::new(key);
        let config_path = home_dir().unwrap().join("nordstone.cfg");
        if config_path.exists() {
            let storage_manager = LocalStorageManager::new(
                config_path, encryptor,
            );
            let data = storage_manager.load();
            self.state = MainState::Decrypted(data);
            return;
        }
        self.state = MainState::Decrypted(Folder::new("NEW FOLDER".into()))
    }

    fn encrypt(&mut self) {
        let encryptor = AgeEncryptor::new(self.key.clone().unwrap());
        let config_path = home_dir().unwrap().join("nordstone.cfg");
        let storage_manager = LocalStorageManager::new(
            config_path, encryptor,
        );
        if let MainState::Decrypted(ref mut data) = self.state {
            storage_manager.save(data)
        }
    }
}

#[derive(Debug)]
enum MainState {
    Decrypted(Folder),
    Encrypted(DecryptForm),
}

#[derive(Debug, Clone)]
enum MainMessage {
    DecryptFormMessage(DecryptFormMessage),
    EditFolder(usize),
    ChangeFolder((usize, String)),
    Save,
    RecordUiMessage((usize, RecordUiMessage)),
}

impl Application for NordstoneUi {
    type Executor = iced::executor::Default;
    type Message = MainMessage;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let form = DecryptForm::new();
        (
            Self {
                state: MainState::Encrypted(form),
                subfolder_to_edit: None,
                record_to_edit: None,
                key: None,
                records: vec![RecordUi {
                    state: RecordUiState::Edit(HashMap::new()),
                    key_to_add: "".to_string(),
                    value_to_add: "".to_string(),
                }],
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        "NORDSTONE".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match self.state {
            MainState::Encrypted(ref mut form) => {
                match message {
                    MainMessage::DecryptFormMessage(msg) => {
                        match msg {
                            DecryptFormMessage::KeyChanged(_) => {
                                form.update(msg.clone());
                                Command::none()
                            }
                            DecryptFormMessage::Decrypt(key) => {
                                self.decrypt(key);
                                Command::none()
                            }
                        }
                    }
                    _ => { Command::none() }
                }
            }
            MainState::Decrypted(ref mut data) => {
                match message {
                    MainMessage::DecryptFormMessage(_) => { Command::none() }
                    MainMessage::EditFolder(index) => {
                        self.subfolder_to_edit = Some(index);
                        if !data.records.is_empty() {
                            self.records = data.records.iter().map(|r| {
                                RecordUi { state: RecordUiState::Edit(r.fields.clone()), key_to_add: "".to_string(), value_to_add: "".to_string() }
                            }).collect();
                        }
                        Command::none()
                    }
                    MainMessage::ChangeFolder((index, new_name)) => {
                        match data.subfolders {
                            Some(ref mut subs) => {
                                subs[index].rename(new_name)
                            }
                            _ => {}
                        }
                        Command::none()
                    }
                    MainMessage::Save => {
                        self.encrypt();
                        self.subfolder_to_edit = None;
                        Command::none()
                    }
                    MainMessage::RecordUiMessage((index, msg)) => {
                        match msg {
                            RecordUiMessage::Save(fields) => {
                                if data.records.is_empty() {
                                    data.records.push(Record {
                                        fields: fields.clone(),
                                        files: None,
                                    })
                                } else { data.records[index].fields = fields.clone(); }

                                self.encrypt();
                                Command::none()
                            }
                            _ => {
                                self.records[index].update(msg);
                                Command::none()
                            }
                        }
                    }
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        match &self.state {
            MainState::Encrypted(form) => {
                row![
                    form.view().map(|msg| { MainMessage::DecryptFormMessage(msg) })
                ].into()
            }
            MainState::Decrypted(data) => {
                match &data.subfolders {
                    Some(subs) => {
                        column(
                            subs.iter().enumerate().map(|(index, s)| {
                                return if Some(index) == self.subfolder_to_edit {
                                    let fields = column(
                                        self.records.iter().enumerate().map(|(index, r)| {
                                            r.view().map(move |m| {
                                                MainMessage::RecordUiMessage((index, m))
                                            })
                                        }).collect()
                                    );
                                    println!("{:?}", self.records);
                                    column![
                                        row![
                                        text_input("input folder name", &s.name).on_input(move |name| {
                                            MainMessage::ChangeFolder((index, name))
                                        }),
                                        button("save").on_press(MainMessage::Save)
                                        ],
                                        row![fields]
                                    ].into()
                                } else {
                                    row![
                                    text(s.name.clone()),
                                    button("edit").on_press(
                                        MainMessage::EditFolder((index))
                                    )
                                    ].into()
                                };
                            }).collect()
                        ).into()
                    }
                    None => text("NO FOLDERS").into()
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum DecryptFormMessage {
    KeyChanged(String),
    Decrypt(String),
}

#[derive(Debug)]
struct DecryptForm {
    key: String,
}

impl DecryptForm {
    fn new() -> Self {
        Self {
            key: "".into()
        }
    }

    fn update(&mut self, message: DecryptFormMessage) {
        match message {
            DecryptFormMessage::KeyChanged(key) => {
                self.key = key
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<DecryptFormMessage> {
        row![
            text_input("input key", &self.key).on_input(|key| {
                DecryptFormMessage::KeyChanged(key)
            }),
            button("decrypt").on_press(DecryptFormMessage::Decrypt(self.key.clone()))
        ].into()
    }
}

#[derive(Debug, Clone)]
enum RecordUiMessage {
    Save(HashMap<String, String>),
    Change(HashMap<String, String>),
    Edit((String, String)),
}

#[derive(Debug, Clone)]
enum RecordUiState {
    Display(HashMap<String, String>),
    Edit(HashMap<String, String>),
}

#[derive(Debug)]
struct RecordUi {
    state: RecordUiState,
    key_to_add: String,
    value_to_add: String,
}

impl RecordUi {
    fn new(fields: HashMap<String, String>) -> Self {
        Self {
            state: RecordUiState::Display(fields),
            key_to_add: "".to_string(),
            value_to_add: "".to_string(),
        }
    }

    fn update(&mut self, message: RecordUiMessage) {
        match self.state {
            RecordUiState::Edit(_) => {
                match message {
                    RecordUiMessage::Save(_) => {}
                    RecordUiMessage::Change(new_data) => {
                        self.state = RecordUiState::Edit(new_data);
                    }
                    RecordUiMessage::Edit((k, v)) => {
                        self.key_to_add = k;
                        self.value_to_add = v;
                    }
                }
            }
            _ => {}
        }
    }

    fn view(&self) -> Element<RecordUiMessage> {
        match &self.state {
            RecordUiState::Display(data) => {
                column(
                    data
                        .iter()
                        .map(|(k, v)| {
                            text(format!("{}:{}", k, v)).into()
                        })
                        .collect()
                ).into()
            }
            RecordUiState::Edit(data) => {
                let existing: Column<RecordUiMessage> = column(
                    data
                        .iter()
                        .map(|(k, v)| {
                            row![
                                text_input("input name", k).on_input(|new_key| {
                                    let mut new_data = data.clone();
                                    new_data.remove(k);
                                    new_data.insert(k.into(), v.into());
                                    RecordUiMessage::Change(new_data)
                                }),
                                text_input("input value", k).on_input(|new_value| {
                                    let mut new_data = data.clone();
                                    new_data.insert(k.into(), new_value);
                                    RecordUiMessage::Change(new_data)
                                })
                            ].into()
                        }).collect()
                ).into();
                let mut new_data = data.clone();
                new_data.insert(self.key_to_add.clone(), self.value_to_add.clone());
                column![
                    existing,
                    row![
                        text_input("input name", &self.key_to_add).on_input(|k| {
                            RecordUiMessage::Edit((k, self.value_to_add.clone()))
                        }),
                        text_input("input value", &self.value_to_add).on_input(|v| {
                            RecordUiMessage::Edit((self.key_to_add.clone(), v))
                        })
                    ],
                    button("save fields").on_press(RecordUiMessage::Save(
                        new_data
                    ))
                ].into()
            }
        }
    }
}

#[tokio::main]
async fn main() -> iced::Result {
    NordstoneUi::run(Settings::default())
}
