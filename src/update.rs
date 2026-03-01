use self_update::backends::github::Update;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn run_update() {
    let status = match Update::configure()
        .repo_owner("obikosato")
        .repo_name("jcal")
        .bin_name(env!("CARGO_PKG_NAME"))
        .show_download_progress(true)
        .current_version(CURRENT_VERSION)
        .build()
        .and_then(|update| update.update())
    {
        Ok(status) => status,
        Err(e) => {
            eprintln!("アップデートに失敗しました: {e}");
            std::process::exit(1);
        }
    };

    if status.updated() {
        println!("v{} にアップデートしました", status.version());
    } else {
        println!("すでに最新バージョン (v{}) です", CURRENT_VERSION);
    }
}
