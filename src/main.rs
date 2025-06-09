use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};

#[derive(Deserialize, Debug)]
struct Response {
    clips: HashMap<String, Clip>,
}
#[derive(Deserialize, Debug)]
struct Clip {
    #[serde(rename = "contentType")]
    content_type: i16,
    #[serde(rename = "contentTitle")]
    content_title: String,
    #[serde(rename = "contentUrl1080p")]
    content_url1080p: String,
    #[serde(rename = "contentUrl720p")]
    content_url720p: String,
    #[serde(rename = "contentUrl480p")]
    content_url480p: String,
    #[serde(rename = "contentUrl360p")]
    content_url360p: String,
    #[serde(rename = "contentUrl240p")]
    content_url240p: String,
    #[serde(rename = "contentUrl144p")]
    content_url144p: String,
    #[serde(rename = "thumbnail1080p")]
    thumbnail_1080p: String,
    #[serde(rename = "thumbnail720p")]
    thumbnail_720p: String,
    #[serde(rename = "thumbnail480p")]
    thumbnail_480p: String,
    #[serde(rename = "thumbnail360p")]
    thumbnail_360p: String,
    #[serde(rename = "thumbnail240p")]
    thumbnail_240p: String,
    #[serde(rename = "thumbnail144p")]
    thumbnail_144p: String,
    #[serde(rename = "sourceWidth")]
    source_width: u32,
    created: u64,
}

fn main() {
    println!("[1] Download a clip\n[2] Download all clips from a profile\n[3] Exit");
    print!("[>] ");
    io::stdout().flush().unwrap();
    loop {
        let input = get_input();
        match input.as_str() {
            "1" => {
                download_clip();
            }
            "2" => {
                println!("This feature is not implemented yet.");
                // download_profile_clips();
            }
            "3" => {
                println!("Exiting the program. Goodbye!"); // thanks to @T4tze for the idea 
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }

        std::thread::sleep(Duration::from_millis(500));
        println!("[1] Download a clip\n[2] Download all clips from a profile\n[3] Exit");
        print!("[>] ");
        io::stdout().flush().unwrap();
    }
}

fn download_clip(){
    print!("Enter clip URL: ");
    io::stdout().flush().unwrap();
    let input = get_input();

    let resp = get(input)
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text");
    let regex = Regex::new(r"var hydrationData=([\s\S]*?)</script>").unwrap();
    let json_data = regex
        .captures(&resp)
        .expect("Failed to find hydration data")
        .get(1)
        .expect("Failed to capture hydration data")
        .as_str()
        .trim()
        .to_string();
    let response: Response =
        serde_json::from_str(&json_data).expect("Failed to parse JSON response");

    if response.clips.is_empty() {
        println!("No clips found in the provided URL.");
        return;
    }

    let clip = response.clips.values().next().unwrap();
    let title = clip.content_title.replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let url = clip.get_best_url().expect("No valid URL found for the clip");
    let filename = format!("{}-{}", title, clip.created);
    let extension = if clip.content_type == 15 { ".mp4" } else { ".jpg" };

    download_file(url, &filename, extension, &clip.created)
        .expect("Failed to download file");
}

fn download_file(
    url: &str,
    filename: &str,
    extension: &str,
    modified: &u64,
) -> std::io::Result<()> {
    let response = get(url).expect("Failed to download file");
    if !response.status().is_success() {
        eprintln!("Failed to download file: {}", response.status());
        return Err(io::Error::new(io::ErrorKind::Other, "Download failed"));
    }

    let mut file = File::create(format!("{}{}", filename, extension))?;
    copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;

    let modified_time = SystemTime::UNIX_EPOCH + Duration::from_secs(*modified);
    file.set_len(file.metadata()?.len())?;
    file.set_modified(modified_time)?;

    println!("Downloaded {} to {}.{}", filename, filename, extension);
    Ok(())
}

fn get_input () -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

impl Clip {
    fn get_best_url(&self) -> Option<&str> {
        let urls = match self.content_type {
            15 => vec![
                (&self.content_url1080p, 1080),
                (&self.content_url720p, 720),
                (&self.content_url480p, 480),
                (&self.content_url360p, 360),
                (&self.content_url240p, 240),
                (&self.content_url144p, 144),
            ],
            29 => vec![
                (&self.thumbnail_1080p, 1080),
                (&self.thumbnail_720p, 720),
                (&self.thumbnail_480p, 480),
                (&self.thumbnail_360p, 360),
                (&self.thumbnail_240p, 240),
                (&self.thumbnail_144p, 144),
            ],
            _ => return None,
        };

        for &(url, width) in &urls {
            if self.source_width >= width && !url.is_empty() {
                return Some(url);
            }
        }

        urls.iter()
            .find(|&&(url, _)| !url.is_empty())
            .map(|&(url, _)| url.as_str())
    }
}
