use regex::Regex;
use reqwest::blocking::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::io::{self, Write};
use std::time::{Duration, SystemTime};
use std::sync::OnceLock;

#[derive(Deserialize, Debug)]
struct Response {
    clips: HashMap<String, Clip>,
}

#[derive(Deserialize, Debug)]
struct ProfileClips(pub Vec<Clip>);

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

pub static DOWNLOAD_DIR: &str = "downloads";
static X_AUTHENTICATION: OnceLock<String> = OnceLock::new();

fn main() {
    if std::fs::create_dir_all(DOWNLOAD_DIR).is_err() {
        eprintln!("Failed to create download directory. Please check permissions.");
        return;
    }

    println!(
        "[1] Download a clip\n[2] Download all clips from a profile\n[3] Change download directory\n[4] Exit"
    );
    print!("[>] ");
    io::stdout().flush().unwrap();
    loop {
        let input = get_input();
        match input.as_str() {
            "1" => {
                download_clip();
            }
            "2" => {
                download_profile_clips();
            }
            "3" => {
                change_download_directory();
            }
            "4" => {
                println!("Exiting the program. Goodbye!"); // thanks to @T4tze for the idea 
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }

        std::thread::sleep(Duration::from_millis(500));
        println!(
            "[1] Download a clip\n[2] Download all clips from a profile\n[3] Change download directory\n[4] Exit"
        );
        print!("[>] ");
        io::stdout().flush().unwrap();
    }
}

fn download_clip() {
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
    let title = clip
        .content_title
        .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
    let url = clip
        .get_best_url()
        .expect("No valid URL found for the clip");
    let filename = format!("{}-{}", title, clip.created);
    let extension = if clip.content_type == 15 {
        ".mp4"
    } else {
        ".jpg"
    };

    download_file(url, &filename, extension, &clip.created).expect("Failed to download file");
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

    let mut file = File::create(format!("{}/{}{}", DOWNLOAD_DIR, filename, extension))?;
    copy(&mut response.bytes().unwrap().as_ref(), &mut file)?;

    let modified_time = SystemTime::UNIX_EPOCH + Duration::from_secs(*modified);
    file.set_len(file.metadata()?.len())?;
    file.set_modified(modified_time)?;

    println!("Downloaded {} to {}.{}", filename, filename, extension);
    Ok(())
}

fn get_input() -> String {
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

fn change_download_directory() {
    print!("[Directory - >] ");
    io::stdout().flush().unwrap();
    let new_dir = get_input();
    if std::fs::create_dir_all(&new_dir).is_ok() {
        println!("Download directory changed to: {}", new_dir);
    } else {
        eprintln!("Failed to change download directory.");
    }
}

fn download_profile_clips() {
    print!("Enter profile URL: ");
    io::stdout().flush().unwrap();
    let profile_url = get_input();
    
    if profile_url.is_empty() {
        println!("Profile URL cannot be empty.");
        return;
    }
    
    if X_AUTHENTICATION.get().is_none() {
        print!("Enter X-Authentication token: ");
        io::stdout().flush().unwrap();
        let x_auth = get_input();
        if x_auth.is_empty() {
            println!("X-Authentication token cannot be empty.");
            return;
        }
        X_AUTHENTICATION.set(x_auth).unwrap();
    }

    let resp = get(&profile_url)
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response text");

    let regex = Regex::new(r#""userId":"(\d+)""#).unwrap();
    let user_id = regex
        .captures(&resp)
        .and_then(|cap| cap.get(1))
        .map_or_else(|| {
            eprintln!("Failed to extract user ID from profile URL.");
            String::new()
        }, |m| m.as_str().to_string());

    if user_id.is_empty() {
        return;
    }
    
    let mut offset = 0;

    loop {
        let url = format!(
            "https://medal.tv/api/content?userId={}&offset={}&limit=100&sortBy=publishedAt&sortDirection=DESC",
            user_id, offset
        );
        let client = reqwest::blocking::Client::new();
        let response: ProfileClips = client
            .get(&url)
            .header("X-Authentication", X_AUTHENTICATION.get().unwrap())
            .send()
            .expect("Failed to send request")
            .json()
            .expect("Failed to parse JSON response");

        if response.0.is_empty() || response.0.len() < 100 {
            println!("No more clips found.");
            break;
        }
        for clip in &response.0 {
            let title = clip
                .content_title
                .replace(['/', '\\', ':', '*', '?', '"', '<', '>', '|'], "_");
            let url = clip
                .get_best_url()
                .expect("No valid URL found for the clip");
            let filename = format!("{}-{}", title, clip.created);
            let extension = if clip.content_type == 15 { ".mp4" } else { ".jpg" };

            if let Err(e) = download_file(url, &filename, extension, &clip.created) {
                eprintln!("Failed to download file: {}", e);
            }
        }
        offset += 100;
        println!("Downloaded {} clips from profile.", response.0.len());
        std::thread::sleep(Duration::from_millis(500)); // not sure what the actual rate limit is so just being safe
    }
    
}