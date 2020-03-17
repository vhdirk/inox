
use tarpc;

#[tarpc::service]
pub trait Page {
    async fn allow_remote_images(name: String) -> ();

    async fn load(html_content: String,
                  css_content: String,
                  part_css: Option<String>,
                  allowed_uris: Vec<String>,
                  use_stdout: bool,
                  use_syslog: bool,
                  disable_log: bool,
                  log_level: String) -> ();

}


