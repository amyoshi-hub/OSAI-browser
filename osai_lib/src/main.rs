use osai_lib::OSAI;

#[tokio::main]
async fn main() {
    let osai = OSAI::new();
    osai.run().await;
}

