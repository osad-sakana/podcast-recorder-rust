use eframe::egui;

fn main() -> eframe::Result<()> {
    // Windowの初期設定
    let native_options = eframe::NativeOptions::default();

    // アプリの起動
    eframe::run_native(
        "Podcast Recorder by Osada",
        native_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            let app = RecorderApp::default();
            Ok(Box::new(app))
        }),
    )
}

// アプリの状態を保持する構造体
struct RecorderApp{
    is_recording: bool,
}

impl Default for RecorderApp {
    fn default() -> Self {
        Self{
            is_recording: false,
        }
    }
}

// 毎フレーム呼ばれる描画処理
impl eframe::App for RecorderApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("ポッドキャスターのためのレコーダー");

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
