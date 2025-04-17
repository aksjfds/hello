use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rfd::FileDialog;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

// 配置结构体，用于存储文件路径
#[derive(Clone)]
struct AppState {
    file_path: PathBuf,
}

// 处理文件下载的路由
async fn download_file(state: web::Data<AppState>) -> impl Responder {
    // 获取文件路径
    let path = &state.file_path;

    // 尝试打开文件
    match File::open(path) {
        Ok(mut file) => {
            // 读取文件内容
            let mut contents = Vec::new();
            if let Err(e) = file.read_to_end(&mut contents) {
                return HttpResponse::InternalServerError()
                    .body(format!("Failed to read file: {}", e));
            }

            // 获取文件名
            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("download");

            // 返回文件响应
            HttpResponse::Ok()
                .append_header((
                    "Content-Disposition",
                    format!("attachment; filename=\"{}\"", filename),
                ))
                .body(contents)
        }
        Err(e) => HttpResponse::NotFound().body(format!("File not found: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 弹出文件选择对话框
    let file_path = FileDialog::new()
        .set_directory(".") // 设置初始目录
        .pick_file()
        .unwrap(); // 打开文件选择对话框

    // 创建应用状态
    let app_state = AppState { file_path };

    // 启动服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/", web::get().to(download_file))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
