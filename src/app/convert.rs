use std::fs::File;
use std::hash::Hash;
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};

use iced::subscription;
use native_dialog::MessageType;
use crate::app::metadata::{FlacMetadata, Metadata, Mp3Metadata};

#[derive(Debug)]
enum ConvertState {
    Start {
        input_path: String,
        output_path: String,
    },
    Converting {
        progress: f32,
        total: usize,
        rest: Vec<PathBuf>,
        output_path: String,
    },
    Error(String),
    End,
}

pub fn start<I: 'static + Hash + Copy + Send + Sync>(
    id: I,
    input_path: String,
    output_path: String,
) -> iced::Subscription<(I, f32, bool)> {
    subscription::unfold(id, ConvertState::Start {
        input_path,
        output_path,
    }, move |state| {
        convert(id, state)
    })
}

async fn convert<Id: Copy>(id: Id, state: ConvertState) -> ((Id, f32, bool), ConvertState) {
    match state {
        ConvertState::Start { input_path, output_path } => {
            println!("Start converting {} to {}", input_path, output_path);
            let result = std::fs::read_dir(input_path).and_then(|folder| {
                let mut files = vec![];
                for entry in folder {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        files.push(path);
                    }
                }
                Ok(files)
            }).and_then(|files| {
                Ok(files.iter().filter(|file| {
                    file.extension().and_then(|ext| {
                        ext.to_str()
                    }).map(|ext| {
                        ext == "ncm"
                    }).unwrap_or(false)
                }).map(|file| {
                    file.clone()
                }).collect::<Vec<_>>())
            }).or_else(|err| {
                Err(err.to_string())
            });

            let progress = async {
                match result {
                    Ok(files) => {
                        println!("Found {} files", files.len());
                        ((id, 0.0, true), ConvertState::Converting { progress: 0.0, total: files.len(), rest: files, output_path })
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        ((id, 0.0, false), ConvertState::Error(err))
                    }
                }
            }.await;
            progress
        }
        ConvertState::Converting { progress, output_path, total, rest } => {
            if rest.is_empty() {
                println!("End converting");
                ((id, 1.0, false), ConvertState::End)
            } else {
                let file = rest.first().unwrap();
                println!("Converting {}", file.to_string_lossy());

                let Ok(buf) = std::fs::read(file) else {
                    return ((id, progress, false), ConvertState::Error("Failed to read file".to_string()));
                };

                let result = ncmdump::Ncmdump::from_reader(Cursor::new(buf))
                    .and_then(|mut ncm| {
                        Ok((ncm.get_data()?, ncm))
                    })
                    .and_then(|(data, mut ncm)| {
                        if let Ok(info)  = ncm.get_info(){
                            let extension = info.format.clone();
                            if extension == "flac" {
                                let image = ncm.get_image().unwrap_or_default();
                                let Ok(data) = FlacMetadata::new(&info, &image, &data)
                                    .inject_metadata(data.clone()) else{
                                    return Ok((data, extension));
                                };
                                Ok((data, extension))
                            }else if extension == "mp3" {
                                let image = ncm.get_image().unwrap_or_default();
                                let Ok(data) = Mp3Metadata::new(&info, &image, &data)
                                    .inject_metadata(data.clone()) else{
                                    return Ok((data, extension));
                                };
                                Ok((data, extension))
                            }else{
                                unreachable!()
                            }
                        } else {
                            unreachable!()
                        }
                    })
                    .or_else(|err| {
                        Err(err.to_string())
                    });

                match result {
                    Ok((data, extension)) => {
                        let output = Path::new(&output_path).join(file.file_stem().unwrap()).with_extension(extension);

                        let mut file = File::options()
                            .create(true)
                            .truncate(true)
                            .write(true)
                            .open(output.clone()).unwrap();


                        let Ok(()) = file.write_all(&data) else {
                            return ((id, progress, false), ConvertState::Error(
                                format!("Failed to create file {}", output.to_string_lossy().to_string())
                            ));
                        };

                        let progress = (total - rest.len() + 1) as f32 / (total as f32);

                        ((id, progress, true), ConvertState::Converting { progress, output_path, total, rest: rest[1..].to_vec() })
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        native_dialog::MessageDialog::new()
                            .set_title("Error Converting")
                            .set_type(MessageType::Error)
                            .set_text(&err)
                            .show_alert()
                            .unwrap();
                        ((id, progress, false), ConvertState::Error(err))
                    }
                }
            }
        }
        ConvertState::End => {
            iced::futures::future::pending().await
        }
        ConvertState::Error(str) => {
            println!("Error: {}", str);
            ((id, 0.0, false), ConvertState::End)
        }
    }
}

