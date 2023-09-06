use anyhow::Result;
use yas::core::genshin::GenshinArtifact;

const MODEL: &[u8] = include_bytes!("../models/model_training.onnx");
const CONTENT: &str = include_str!("../models/index_2_word.json");

fn main() -> Result<()> {
    yas::init_env(yas::Game::Genshin);

    let mut scanner = yas::get_scanner(MODEL, CONTENT);

    let results = scanner.scan()?;

    let artifacts = yas::map_results_to::<GenshinArtifact>(&results);

    println!("{:#?}", artifacts);

    Ok(())
}

#[cfg(test)]
mod tests {
    use yas::core::inference::CRNNModel;

    use super::*;

    #[test]
    fn test() {
        yas::init_env(yas::Game::Genshin);

        CRNNModel::new(MODEL, CONTENT).expect("Failed to load model");
    }
}
