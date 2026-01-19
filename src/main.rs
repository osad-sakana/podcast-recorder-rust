use eframe::egui;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
use std::sync::{Arc, Mutex};

fn main() -> eframe::Result<()> {
    print_input_devices();

    // Windowの初期設定
    let native_options = eframe::NativeOptions::default();

    // audio_dataの初期化
    let audio_data: Arc<Mutex<Vec<f32>>> = Arc::new(Mutex::new(Vec::new()));
    let audio_data_for_thread = Arc::clone(&audio_data);
    let is_recording: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    let is_recording_for_thread = Arc::clone(&is_recording);


    std::thread::spawn(move || {
        println!("録音スレッド開始");
        let device = cpal::default_host().default_input_device().expect("デバイスの取得に失敗");
        let config = device.default_input_config().expect("設定の取得に失敗");
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let is_recording = if let Ok(lock) = is_recording_for_thread.lock(){
                    *lock
                }else{
                    false
                };

                if is_recording{
                    if let Ok(mut storage) = audio_data_for_thread.lock(){
                        // 届いた音データをベクタに追加
                        storage.extend_from_slice(data);
                    }
                }
            },
            move |err| {
                eprintln!("録音エラー: {}", err);
            },
            None
        ).expect("ストリームの作成に失敗");

        stream.play().expect("ストリームの再生に失敗");

        loop{
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let app = RecorderApp{
        audio_data: Arc::clone(&audio_data), // appに共有データを渡す
        is_recording_shared: Arc::clone(&is_recording),
        ..RecorderApp::default()
    };

    // アプリの起動
    eframe::run_native(
        "Podcast Recorder by Osada",
        native_options,
        Box::new(move |cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

// アプリの状態を保持する構造体
struct RecorderApp{
    is_recording: bool,
    recording_title: String,
    audio_data: Arc<Mutex<Vec<f32>>>,
    is_recording_shared: Arc<Mutex<bool>>,
}

impl Default for RecorderApp {
    fn default() -> Self {
        Self{
            is_recording: false,
            recording_title: "エピソード名未設定".to_owned(),
            audio_data: Arc::new(Mutex::new(Vec::new())),
            is_recording_shared: Arc::new(Mutex::new(false)),
        }
    }
}

// 毎フレーム呼ばれる描画処理
impl eframe::App for RecorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("ポッドキャスターのためのレコーダー");

            // エピソード名の入力
            ui.add_enabled_ui(!self.is_recording, |ui|{
                ui.horizontal(|ui|{
                    ui.label("エピソード名");
                    ui.text_edit_singleline(&mut self.recording_title);
                });
            });


            ui.separator();

            // ここにUIを追加
            if ui.button("録音を開始する").clicked(){
                self.is_recording = true;
            }

            if self.is_recording {
                ui.label("現在: 録音しています。");
            }
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // 1. フォントデータを読み込む
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/NotoSansJP-Medium.ttf")),
    );

    // 2. 優先順位の最上位に日本語フォントを追加する
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // 3. コンテキストに変換
    ctx.set_fonts(fonts);
}

fn print_input_devices(){
    let host = cpal::default_host();
    let devices = host.input_devices().expect("入力デバイスが見つかりません。");

    println!("利用可能なマイク一覧: ");
    for device in devices {
        let name = device
            .description()
            .map(|d| d.name().to_string())
            .unwrap_or_else(|_|{
                "不明なデバイス".to_string()
            });

        println!("  - {}", name);
    }
}
