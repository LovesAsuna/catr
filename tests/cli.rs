use assert_cmd::Command;
use predicates::prelude::predicate;
use rand::distributions::Alphanumeric;
use rand::Rng;

const PRG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const FOX: &str = "tests/inputs/fox.txt";
const SPIDERS: &str = "tests/inputs/spiders.txt";
const BUSTLE: &str = "tests/inputs/the-bustle.txt";

#[test]
fn skips_bad_file() -> anyhow::Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

fn gen_bad_file() -> String {
    loop {
        let file_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if std::fs::metadata(&file_name).is_err() {
            return file_name;
        }
    }
}
