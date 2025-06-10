# MedalTV Downloader Rusted
MedalClipDownloader is a simple and easy to use tool to download clips and screenshots from medal.tv WITHOUT the watermark.

## How to Use
1. Run the program.
2. Select from the main menu:
  - **[1] Download a Clip**: Downloads a clip based off of a link.
  - **[2] Download All Profile Clips**: Downloads all the clips from a profile. (Requires an authentication token.)
## How to Get Your X-Authentication Token
1. Login to https://medal.tv
2. Go to any profile with developer tools open and the network tab selected
3. Look for `publishing?limit=50`, click it and look at the request headers for `X-Authentication`
4. Copy the value and use it.
![SBZs1PDmrNs6vzL8OPY01109](https://github.com/user-attachments/assets/bcf68124-fbbd-4dd0-955e-5e16400392bc)


## Getting Started
You can either build it or use the [pre-compiled binaries in releases](https://github.com/whoswhip/MedalClipDownloader/releases)
### Building
1. **Prerequisites**  
Make sure you have Rust installed.  
```sh
rustc --version
cargo --version
```  
If not, get it from [https://rustup.rs](https://rustup.rs).

2. **Clone the repository**  
```sh 
git clone --branch mtv-rusted --single-branch https://github.com/whoswhip/MedalClipDownloader.git mtv-rusted
cd mtv-rusted
```

3. **Build the project**  
```sh 
cargo build --release
```

4. **Run the project**  
```sh 
cargo run
```
