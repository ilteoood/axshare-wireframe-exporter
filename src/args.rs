use std::env;

pub struct Args {
    pub base_url: String,
    pub pdf_name: String,
}

impl Args {
    pub fn from_args() -> Args {
        let args = env::args().collect::<Vec<String>>();
        let base_url = args[1].clone();
        Args {
            base_url: base_url.trim_end_matches("/").to_string(),
            pdf_name: args
                .get(2)
                .unwrap_or(&"axshare-wireframe-exporter.pdf".to_string())
                .clone(),
        }
    }
}
