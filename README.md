# MedalClipDownloader
MedalClipDownloader is a simple and easy to use tool to download clips and screenshots from medal.tv WITHOUT the watermark.
## How to Use
1. Run the program.
2. Select from the main menu:
  - **[1] Download All Profile Clips**: Downloads all the clips from a profile. (Requires an authentication token.)
  - **[2] Download a Clip**: Downloads a clip based off of a link.
## How to Get Your X-Authentication Token
1. Login to https://medal.tv
2. Go to any profile with developer tools open and the network tab selected
3. Look for `publishing?limit=50`, click it and look at the request headers for `X-Authentication`
4. Copy the value and use it.
![SBZs1PDmrNs6vzL8OPY01109](https://github.com/user-attachments/assets/bcf68124-fbbd-4dd0-955e-5e16400392bc)

