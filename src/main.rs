use regex;
use reqwest::{self};
use std::error::Error;
use std::fs::{self};
use std::io::{self, Write};

#[derive(Debug)]
struct Chapter {
    title: String,
    link: String,
}

#[derive(Debug)]
struct Novel {
    // novelid: String,
    link: String,
    chapters: Vec<Chapter>,
}

impl Novel {
    fn new(novelid: &str) -> Result<Self, Box<dyn Error>> {
        let mut novel = Novel {
            // novelid: novelid.to_string(),
            link: format!("http://www.jjwxc.net/onebook.php?novelid={}", novelid),
            chapters: Vec::new(),
        };

        let bytes = reqwest::blocking::get(&novel.link)?.bytes()?;
        let (body, _, _) = encoding_rs::GBK.decode(&bytes);
        let text = body.to_string();

        let re = regex::Regex::new(r#".*novelid=\d+&chapterid=(\d+)">(.*)</a>"#)?;
        for cap in re.captures_iter(&text) {
            let chapterid = cap.get(1).unwrap().as_str().to_string();
            let title = cap.get(2).unwrap().as_str().to_string();
            novel.chapters.push(Chapter {
                title: title,
                link: format!(
                    "http://www.jjwxc.net/onebook.php?novelid={}&chapterid={}",
                    novelid, chapterid
                ),
            });
        }
        Ok(novel)
    }

    fn download(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let mut content = String::from("");

        for (index, &ref chapter) in self.chapters.iter().enumerate() {
            let bytes = reqwest::blocking::get(&chapter.link)?.bytes()?;
            let (body, _, _) = encoding_rs::GBK.decode(&bytes);
            let text = body.to_string();

            let re =
                regex::Regex::new(r#"(?s)<div style="clear:both;"></div>(.*?)<div id="#).unwrap();

            for cap in re.captures_iter(&text) {
                if let Some(matched) = cap.get(1) {
                    content = format!("{}\n{}\n{}", content, chapter.title, matched.as_str());
                }
            }

            let progress = (index + 1) as f32 / self.chapters.len() as f32 * 100.0;
            print!("\rDownloading...{:.2}%", progress);
            io::stdout().flush().unwrap();
        }

        content = content.replace("<br>", "\n");

        let filepath = std::path::Path::new(filepath);
        if filepath.exists() {
            fs::remove_file(filepath)?;
        }
        let mut file = fs::File::create(filepath)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let novel = Novel::new("7434574")?;
    novel.download("novel.txt")
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_get_html() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
