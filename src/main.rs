use std::{path::Path, path::PathBuf};
use regex::Regex;
use anyhow::*;
use colored::Colorize;
use clap::{App, Arg};
use std::fs;
use globset::{Glob, GlobMatcher};

trait Rgrep {
    fn search(&mut self, str: &str) -> &Self;
    fn read(&mut self) -> &mut Self;
    fn print_result(&self);
}

#[derive(Default, Debug)]
struct FileGrep {
    value: String,
    path: PathBuf,
    file_name: String,
    result: Vec<Vec<String>>,
}

impl FileGrep {
    fn new(path: PathBuf, file_name: String) -> Self {
        Self {
            path,
            file_name,
            ..Default::default()
        }
    }
}

impl Rgrep for FileGrep {
    fn search(&mut self, str: &str) -> &Self {
        let mut result = Vec::new();

        for (row, line) in self.value.lines().enumerate() {
            if let Some(mat) = Regex::new(str).unwrap().find(line) {
                let s_i = mat.start();
                let e_i = mat.end();
                let line_res = vec![
                    format!("{}:{}", row + 1, s_i),
                    line[..s_i].to_string(),
                    line[s_i..e_i].to_string(),
                    line[e_i..].to_string()
                ];
                result.push(line_res);
            }
        }
        self.result = result;
        self
    }
    fn print_result(&self) {
        if !self.result.is_empty() {
            println!("{}", self.file_name.yellow());
        }
        for line in self.result.iter() {
            println!("{} {}{}{}", line[0].as_str().blue(), line[1], line[2].red(), line[3]);
        }
    }
    fn read(&mut self) -> &mut Self {
        let content = fs::read_to_string(self.path.clone()).unwrap();
        self.value = content;
        self
    }
}

fn visit_dirs(dir: &Path, match_str: &str, glob: &GlobMatcher) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, match_str, glob)?;
            } else {
                if glob.is_match(entry.path()) {
                    FileGrep::new(entry.path(), entry.file_name().into_string().unwrap())
                        .read()
                        .search(match_str)
                        .print_result();
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("rgrep")
        .version("v0.1")
        .arg(
            Arg::new("match_str")
                .takes_value(true)
        )
        .arg(
            Arg::new("file_path")
                .takes_value(true)
        )
        .get_matches();
    let match_str = matches.value_of("match_str").unwrap();
    let file_path = matches.value_of("file_path").unwrap();
    let glob = Glob::new(file_path)?.compile_matcher();
    visit_dirs(Path::new("."), match_str, &glob)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basi_interface() -> Result<()> {
        FileGrep::new("./a.md".into(), "a.md".into())
            .read()
            .search("Hel[^\\s]+")
            .print_result();
        // let file_grep  = file_str.search("Hello");
        Ok(())
    }
}