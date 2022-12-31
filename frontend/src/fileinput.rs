use gage_study::data::Data;
use gloo::file::callbacks::FileReader;
use gloo::file::File;
use serde_json;
use std::collections::HashMap;
use web_sys::{DragEvent, Event, FileList, HtmlInputElement};
use yew::html::TargetCast;
use yew::{html, Callback, Component, Context, Html};

pub struct FileInfo {
    name: String,
    file_type: String,
    data: String,
}

pub struct InputFileReader {
    readers: HashMap<String, FileReader>,
    files: Vec<FileInfo>,
}

pub enum Msg {
    Loaded(String, String, String),
    Files(Vec<File>),
}

impl Component for InputFileReader {
    type Properties = ();
    type Message = Msg;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            readers: HashMap::default(),
            files: Vec::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Loaded(file_name, file_type, data) => {
                self.files.push(FileInfo {
                    data,
                    file_type,
                    name: file_name.clone(),
                });
                self.readers.remove(&file_name);
                true
            }
            Msg::Files(files) => {
                for file in files.into_iter() {
                    let file_name = file.name();
                    let ext = std::path::Path::new(&file_name)
                        .extension()
                        .map(std::ffi::OsStr::to_str)
                        .expect("Failed to get file extension");
                    let file_type = match ext {
                        Some(t) => t.to_string(),
                        None => "".to_string(),
                    };
                    let task = {
                        let link = ctx.link().clone();
                        let file_name = file_name.clone();
                        gloo::file::callbacks::read_as_text(&file, move |res| {
                            link.send_message(Msg::Loaded(
                                file_name,
                                file_type,
                                res.expect("failed to read file"),
                            ))
                        })
                    };
                    self.readers.insert(file_name, task);
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="wrapper">
                <p id="title">{ "Upload Your Files To The Cloud" }</p>
                <label for="file-upload">
                    <div
                        id="drop-container"
                        ondrop={ctx.link().callback(|event: DragEvent| {
                            event.prevent_default();
                            let files = event.data_transfer().unwrap().files();
                            Self::upload_files(files)
                        })}
                        ondragover={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                        ondragenter={Callback::from(|event: DragEvent| {
                            event.prevent_default();
                        })}
                    >
                        <i class="fa fa-cloud-upload"></i>
                        <p>{"Drop your images here or click to select"}</p>
                    </div>
                </label>
                <input
                    id="file-upload"
                    type="file"
                    accept=".csv,.json"
                    multiple={true}
                    onchange={ctx.link().callback(move |e: Event| {
                        let input: HtmlInputElement = e.target_unchecked_into();
                        Self::upload_files(input.files())
                    })}
                />
                <div id="preview-area">
                    { for self.files.iter().map(Self::view_file) }
                </div>
            </div>
        }
    }
}

impl InputFileReader {
    fn view_file(file: &FileInfo) -> Html {
        let datas: Vec<Data> = serde_json::from_str(&file.data).expect("Failed to deserialize.");
        html! {
            <div class="preview-tile">
                <p class="preview-name">{ format!("{}", file.name) }</p>
                <div class="preview-media">
                    if file.file_type != "".to_string() {
                        <div style="width:500px;height:200px;overflow-x:hidden;overflow-y:auto;text-align:justify;">
                            <table>
                                <tr>
                                    <th>{"Part"}</th>
                                    <th>{"Operator"}</th>
                                    <th>{"Replicate"}</th>
                                </tr>
                                {
                                    datas.into_iter().map(|data| {
                                        html!{
                                            <tr>
                                                <td>{ data.part }</td>
                                                <td>{ data.operator }</td>
                                                <td>{ data.replicate }</td>
                                            </tr>
                                        }
                                    }).collect::<Html>()
                                }
                            </table>
                        </div>
                    }
                </div>
            </div>
        }
    }

    fn upload_files(files: Option<FileList>) -> Msg {
        let mut result = Vec::new();

        if let Some(files) = files {
            let files = js_sys::try_iter(&files)
                .unwrap()
                .unwrap()
                .map(|v| web_sys::File::from(v.unwrap()))
                .map(File::from);
            result.extend(files);
        }
        Msg::Files(result)
    }
}
