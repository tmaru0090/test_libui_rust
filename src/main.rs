// cargo-deps:libui

//プロジェクト作成のサポートをするための簡易的なコード
mod common;
mod define;
extern crate libui;
use common::*;
use libui::controls::*;
use libui::prelude::*;
use libui::draw::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{self,Read,Write,BufReader};
use tokio::time::Sleep;
use std::time::Duration;
use futures_timer::Delay;

#[derive(Serialize, Deserialize)]
pub struct ProjectData {
    pub meta: ProjectMetaData,
    pub langauge: ProjectLangaugeData,
    pub sample: ProjectSampleData,
}
impl ProjectData {
    fn new() -> ProjectData {
        return ProjectData {
            meta: ProjectMetaData::new(),
            langauge: ProjectLangaugeData::new(),
            sample: ProjectSampleData::new(),
        };
    }
}
#[derive(Serialize, Deserialize)]
pub struct ProjectMetaData {
    pub path: String,
}
impl ProjectMetaData {
    fn new() -> ProjectMetaData {
        return ProjectMetaData {
            path: String::new(),
        };
    }
}
#[derive(Serialize, Deserialize)]
pub struct ProjectLangaugeData {
    pub selected_langauge_index: i32,
    pub selected_langauge_type_index: i32,
}
impl ProjectLangaugeData {
    fn new() -> ProjectLangaugeData {
        return ProjectLangaugeData {
            selected_langauge_index: 0,
            selected_langauge_type_index: 0,
        };
    }
}
#[derive(Serialize, Deserialize)]
pub struct ProjectSampleData {
    pub create_sample_code: bool,
}
impl ProjectSampleData {
    fn new() -> ProjectSampleData {
        return ProjectSampleData {
            create_sample_code: false,
        };
    }
}

fn yaml_print_data(path: &str) -> Result<(), serde_yaml::Error> {
    let mut file = std::fs::File::open(path).expect("Could not read files");
    let mut project_data: ProjectData = serde_yaml::from_reader(file).expect("err!");
    println!("path:{}", project_data.meta.path);
    println!(
        "langauge_selected_index:{}",
        project_data.langauge.selected_langauge_index
    );
    println!(
        "langauge_selected_type_index:{}",
        project_data.langauge.selected_langauge_type_index
    );
    println!(
        "crate_sample_code:{}",
        project_data.sample.create_sample_code
    );
    return Ok(());
}
fn yaml_write_data_tostring(data: String) {
    let mut project_data:ProjectData = ProjectData::new();
    project_data.sample.create_sample_code = false;
    project_data.langauge.selected_langauge_index = 190;
    let mut file =  std::fs::File::open("./project.yaml").unwrap();
    let mut writer = std::io::BufWriter::new(file.try_clone().unwrap());
    serde_yaml::to_writer(&mut writer,&project_data).unwrap();
    //file.write_all(data.as_bytes());
}
fn main() {
    /*
    let mut cmd_cargo = std::process::Command::new("cargo");
    cmd_cargo.output().expect("Faild Execute");
    println!("cmd_status:{}",cmd_cargo.status().unwrap());
    */
    let ui = libui::UI::init().expect("libui:init_error!");
    let mut window = Window::new(
        &ui,
        WINDOW_TITLE,
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowType::HasMenubar,
    );
    let mut verbox = VerticalBox::new();
    let mut val = 0;
    let mut progress_bar = ProgressBar::new();
    progress_bar.set_value(val);
    
    let mut file_pick_path = String::new();
    let mut project_bt = Button::new("プロジェクトを作成");
    let mut file_pick_label = Label::new("");
    let mut file_select_bt = Button::new("ファイルを選択");
    let mut file_select_label = Label::new("プロジェクトを作成するフォルダを選択してください");
    let mut layout = VerticalBox::new();
    let langauge_label = Label::new("言語を選択してください");
    let langauge_type_label = Label::new("言語の種類を選択してください");
    let sample_yes_no_label = Label::new("サンプルコードを生成しますか？");
    let mut sample_yes_no_combo = Combobox::new();
    let sample_label = Label::new("サンプルを選択してください");
    let mut sample_combo = Combobox::new();
    let langauge_type_combo = Combobox::new();
    let langauge_combo = Combobox::new();
       //ドロップダウンのそれぞれの要素に値を追加
    langauge_combo.append("Python");
    langauge_combo.append("C/C++");
    langauge_combo.append("Rust");

    langauge_type_combo.append("binary");
    langauge_type_combo.append("library");

    sample_yes_no_combo.append("はい");
    sample_yes_no_combo.append("いいえ");

    sample_combo.append("サンプル1:ハローワールド");
    sample_combo.append("サンプル1:ウィンドウ生成");
     // サンプル選択は通常オフ
    sample_combo.disable();

    // レイアウトにそれぞれの要素を追加
    
    layout.append(progress_bar.clone(), LayoutStrategy::Compact);
    // 最初の余白を追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);

    // プロジェクト作成用のフォルダーの説明を追加
    layout.append(file_select_label, LayoutStrategy::Compact);
    // プロジェクト作成用のフォルダーのボタンを追加
    layout.append(file_select_bt.clone(), LayoutStrategy::Compact);
    // 選択されたファイルのパスを表示
    layout.append(file_pick_label.clone(), LayoutStrategy::Compact);
    // 余白を追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);

    // 言語選択の説明を追加
    layout.append(langauge_label, LayoutStrategy::Compact);
    // 言語を選択するためのドロップダウンリストを追加
    layout.append(langauge_combo, LayoutStrategy::Compact);

    // 余白を追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);

    // 言語の種類の説明を追加
    layout.append(langauge_type_label, LayoutStrategy::Compact);
    // 言語の種類を選択するためののドロップダウンリストを追加
    layout.append(langauge_type_combo, LayoutStrategy::Compact);

    //余白の追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);

    // サンプルコードを生成するかどうかの説明を追加
    layout.append(sample_yes_no_label, LayoutStrategy::Compact);
    // はいかいいえを選択するためののドロップダウンリストを追加
    layout.append(sample_yes_no_combo.clone(), LayoutStrategy::Compact);

    // 余白の追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);
    // サンプルを選択するかの説明を追加
    layout.append(sample_label.clone(), LayoutStrategy::Compact);
    // サンプルを選択するためのドロップダウンを追加
    layout.append(sample_combo.clone(), LayoutStrategy::Compact);

    // 余白を追加
    layout.append(Spacer::new(), LayoutStrategy::Stretchy);
    // プロジェクトの作成用のボタンを追加
    layout.append(project_bt.clone(), LayoutStrategy::Compact);

    // ファイルセレクトのボタンを押されたとき
    file_select_bt.on_clicked({
        // 所有権を移動してしまうため、値をコピーして使う
        let ui = ui.clone();
        let mut window = window.clone();
        move |_| {
            if let Some(path) = window.open_folder() {
                let mut path_str:String = String::new();
                let mut res_str:String = String::new();
                path_str = path.to_str().unwrap().to_string();
                res_str = "プロジェクトを作成するフォルダー:".to_owned()+path.to_str().unwrap();
                println!("file_pick:{}", &path_str);
                file_pick_label.set_text(&res_str);
            } else {
                println!("file_pick:not_pickd");
            }
        }
    });
    // プロジェクトの作成のボタンを押されたとき
    project_bt.on_clicked({

        // 所有権を移動してしまうため、値をコピーして使う
        let ui = ui.clone();
        let mut window = window.clone();
        let mut val = 0;
        move |_| {
            progress_bar.set_value(val);
            //let mut file =std::fs::File::create("./project.yaml");
            //yaml_print_data("./project.yaml").expect("");
            yaml_write_data_tostring("".to_string());
            yaml_print_data("./project.yaml");
        }
    });
    sample_yes_no_combo.on_selected(&ui, {
        let ui = ui.clone();
        let sample_yes_no_combo = sample_yes_no_combo.clone();
        let mut layout = layout.clone();
        move |_| {
            let select_index = sample_yes_no_combo.selected();
            match select_index {
                0 => {
                    sample_label.clone().enable();
                }
                1 => {
                    sample_label.clone().disable();
                }
                _ => {}
            }
        }
    });
    // レイアウトのセット
    window.set_child(layout.clone());
    // ウィンドウの表示
    window.show();
    // UIの表示
    ui.main();
}
