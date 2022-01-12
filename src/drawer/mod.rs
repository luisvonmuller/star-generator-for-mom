use crate::Student;
use std::env;

const PATH: &str = "mockup_wizards.svg";
use std::fs;

pub fn process(info: &Student) {
    let mut content = fs::read_to_string(format!(
        "{}/{}",
        env::current_dir().unwrap().display(),
        PATH
    ))
    .unwrap();

    content = content.replace("Nome Aluno Place", &info.name.trim());
    content = content.replace("Livro X", &info.book.trim());
    fs::write(format!("./estrelas/{}.svg", info.name.trim()), content).unwrap();
}
