using System.Text.RegularExpressions;
using Newtonsoft.Json.Linq;
using System.Diagnostics;


class Program
{
    static string X_Authentication = "";
    static long totalBytesDownloaded = 0;
    static double totalElapsedTime = 0;
    static async Task Main(string[] args)
    {
        Console.Clear();
        Console.Title = "Medal.tv Clip Downloader";
        string[] options = new string[] { "[1] Download All Profile Clips", "[2] Download a Clip" };
        PrintMenu(options);
        string choice = Console.ReadKey().KeyChar.ToString();
        Console.WriteLine();
        switch (choice)
        {
            case "1":
                await DownloadAllClips();
                break;
            case "2":
                await DownloadClip();
                break;
            default:
                Console.WriteLine("Invalid choice.");
                break;
        }
    }
    static async Task DownloadAllClips()
    {
        Console.Clear();
        PrintMenu(new string[] { "Enter the profile link" });
        string profileLink = Console.ReadLine();
        PrintMenu(new string[] { "Enter your X-Authentication Token" });
        X_Authentication = Console.ReadLine();

        var client = new HttpClient();

        string html = await client.GetStringAsync(profileLink);
        var regex = new Regex(@"(?<=""userId"":"")\d+");
        var match = regex.Match(html);
        string userId = match.Value;
        long offset = 0;
        bool finished = false;
        try
        {
            while (!finished)
            {
                var request = new HttpRequestMessage
                {
                    Method = HttpMethod.Get,
                    Headers =
                    {
                        { "User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:128.0) Gecko/20100101 Firefox/128.0" },
                        { "X-Authentication", X_Authentication },
                    },
                };
                request.RequestUri = new Uri($"https://medal.tv/api/content?userId={userId}&offset={offset}&sortBy=publishedAt&sortDirection=DESC");
                var response = await client.SendAsync(request);
                var body = await response.Content.ReadAsStringAsync();
                var json = JArray.Parse(body);
                offset = json.Count;
                int i = 0;
                if (json.Count == 0)
                    finished = true;
                foreach (var clip in json)
                {
                    i++;
                    var contentUrl1080p = clip["contentUrl1080p"].ToString();
                    var contentUrl720p = clip["contentUrl720p"].ToString();
                    var contentUrl480p = clip["contentUrl480p"].ToString();
                    var videoLengthSeconds = clip["videoLengthSeconds"].ToString();

                    string contentId = clip["contentId"].ToString();
                    string contentTitle = clip["contentTitle"].ToString().Replace(" ", "_").Replace(@"""", "");

                    if (contentTitle.Contains("Instant_Screenshot") && videoLengthSeconds == "1")
                    {
                        contentUrl1080p = clip["thumbnail1080p"].ToString();
                        contentUrl720p = clip["thumbnail720p"].ToString();
                        contentUrl480p = clip["thumbnail480p"].ToString();
                    }

                    if (await DownloadURL(contentUrl1080p, contentTitle, contentId, "1080p", i, offset))
                        continue;
                    else if (await DownloadURL(contentUrl720p, contentTitle, contentId, "720p", i, offset))
                        continue;
                    else if (await DownloadURL(contentUrl480p, contentTitle, contentId, "480p", i, offset))
                        continue;
                }
            }
        }
        catch (Exception e)
        {
            Console.BackgroundColor = ConsoleColor.Red;
            Console.ForegroundColor = ConsoleColor.Black;
            Console.WriteLine(e.Message);
            Console.ResetColor();
        }
        Console.Title = $"Downloaded {totalBytesDownloaded / 1024 / 1024} MB in {totalElapsedTime:0.00} seconds with an average download speed of {(totalBytesDownloaded / 1024d / 1024d) / totalElapsedTime:0.00} MB/s";
        Thread.Sleep(1000);
        Task.Run(() =>
        {
            Main(null);
        });
    }
    static async Task DownloadClip()
    {
        Console.Clear();
        PrintMenu(new string[] { "Enter the clip link" });
        string clipLink = Console.ReadLine();
        var client = new HttpClient();
        string html = await client.GetStringAsync(clipLink);
        var regex = new Regex(@"(?<=var hydrationData=)[\s\S]*?(?=</script>)");
        var match = regex.Match(html);
        regex = new Regex(@"(?<=""contentId"":"")[\s\S]*?(?="")");
        var cidmatch = regex.Match(match.Value);
        string contentIdd = cidmatch.Value;
        string json = match.Value;
        JObject jjson = JObject.Parse(json);
        JObject clips = JObject.Parse(jjson["clips"].ToString());
        JObject clip = JObject.Parse(clips[contentIdd].ToString());
        string contentUrl1080p = clip["contentUrl1080p"].ToString();
        string contentUrl720p = clip["contentUrl720p"].ToString();
        string contentUrl480p = clip["contentUrl480p"].ToString();
        string videoLengthSeconds = clip["videoLengthSeconds"].ToString();
        string contentId = clip["contentId"].ToString();
        string contentTitle = clip["contentTitle"].ToString().Replace(" ", "_").Replace(@"""", "");

        if (contentTitle.Contains("Instant_Screenshot") && videoLengthSeconds == "1")
        {
            contentUrl1080p = clip["thumbnail1080p"].ToString();
            contentUrl720p = clip["thumbnail720p"].ToString();
            contentUrl480p = clip["thumbnail480p"].ToString();
        }
        

        if (await DownloadURL(contentUrl1080p, contentTitle, contentId, "1080p", 1, 1))
        {
            Thread.Sleep(1000);
            await Main(null);
        }
        else if (await DownloadURL(contentUrl720p, contentTitle, contentId, "720p", 1, 1))
        {
            Thread.Sleep(1000);
            await Main(null);
        }
        else if (await DownloadURL(contentUrl480p, contentTitle, contentId, "480p", 1, 1))
        {
            Thread.Sleep(1000);
            await Main(null);
        }


    }
    static async Task<bool> DownloadURL(string url, string contentTitle, string contentId, string quality, long index, long max)
    {
        if (!Directory.Exists("Clips"))
            Directory.CreateDirectory("Clips");
        try
        {
            string fileextension = url.Split('.').Last();
            fileextension = fileextension.Substring(0, fileextension.IndexOf("?"));
            var client = new HttpClient();
            var response = await client.GetAsync(url, HttpCompletionOption.ResponseHeadersRead);
            var totalBytes = response.Content.Headers.ContentLength ?? -1L;
            var canReportProgress = totalBytes != -1;

            using (var stream = await response.Content.ReadAsStreamAsync())
            {
                var progress = new Progress<(long bytesDownloaded, double speed)>(report =>
                {
                    var (bytesDownloaded, speed) = report;
                    if (canReportProgress)
                    {
                        Console.Write($"\r{contentTitle}.{fileextension}({index}/{max}): {bytesDownloaded}/{totalBytes} bytes ({(bytesDownloaded / (double)totalBytes) * 100:0.00}%) - Speed: {speed:0.00} MB/s                          ");
                    }
                    else
                    {
                        Console.Write($"\r{contentTitle}.{fileextension}({index}/{max}): {bytesDownloaded} bytes - Speed: {speed:0.00} MB/s                                                                                              ");
                    }
                });

                using (var fileStream = new FileStream($"Clips/{contentTitle}_{contentId}_{quality}.{fileextension}", FileMode.Create, FileAccess.Write, FileShare.None))
                {
                    var stopwatch = Stopwatch.StartNew();
                    await CopyToAsync(stream, fileStream, 81920, progress);
                    stopwatch.Stop();

                    totalBytesDownloaded += totalBytes;
                    totalElapsedTime += stopwatch.Elapsed.TotalSeconds;
                }
            }
            Console.WriteLine();
            return true;
        }
        catch (Exception e)
        {
            Console.BackgroundColor = ConsoleColor.Red;
            Console.ForegroundColor = ConsoleColor.Black;
            Console.WriteLine($"Failed to download {contentTitle}_{contentId}_{quality}.mp4\nReason: {e.Message}");
            Console.ResetColor();
            return false;
        }
    }

    static async Task CopyToAsync(Stream source, Stream destination, int bufferSize, IProgress<(long, double)> progress = null)
    {
        var buffer = new byte[bufferSize];
        long totalBytesRead = 0;
        int bytesRead;
        var stopwatch = Stopwatch.StartNew();

        while ((bytesRead = await source.ReadAsync(buffer, 0, buffer.Length)) != 0)
        {
            await destination.WriteAsync(buffer, 0, bytesRead);
            totalBytesRead += bytesRead;
            var elapsedSeconds = stopwatch.Elapsed.TotalSeconds;
            var speed = (totalBytesRead / 1024d / 1024d) / elapsedSeconds; 
            progress?.Report((totalBytesRead, speed));
        }
    }

    static void PrintMenu(string[] options)
    {
        // ┌┐└┘├┤┬┴┼│─» ►
        string menu = "";
        int longestoption = 0;

        for (int i = 0; i < options.Length; i++)
        {
            if (options[i].Length > longestoption)
                longestoption = options[i].Length;
        }
        menu += "┌";
        for (int i = 0; i < longestoption + 2; i++)
        {
            menu += "─";
        }
        menu += "┐\n";
        for (int i = 0; i < options.Length; i++)
        {
            menu += "│ ";
            menu += options[i];
            for (int j = 0; j < longestoption - options[i].Length; j++)
            {
                menu += " ";
            }
            menu += " │\n";
        }
        menu += "├";
        for (int i = 0; i < longestoption + 2; i++)
        {
            menu += "─";
        }
        menu += "┘\n";
        menu += "└[»] ";
        Console.Write(menu);
    }
}