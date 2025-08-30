
using OpenQA.Selenium;
using OpenQA.Selenium.DevTools.V132.Preload;
using OpenQA.Selenium.Firefox;
using System;
using System.Diagnostics;
using System.Globalization;
using System.Reflection;
using System.Text;

using Timer = System.Timers.Timer;
namespace CLI
{
    public class Utils : IDisposable
    {
        static string helpMessage = """
            
            Welcome to DU CLI, the WSB Download Utility CLI Application

            Valid Arguments:
            'download' - Download a specific file. 
            'help' - Displays this message.

            Valid Usage:
            --> wsb.exe download amd
            --> wsb.exe download roblox
            --> wsb.exe download vivaldi
            """;
        static List<string> ValidArgs = ["download", "help"];
        static DateTime startTime = DateTime.Now;
        static DateTime endTime;

        #region Argument Functions
        public static bool IsValidArg(string arg)
        { 
            return ValidArgs.Contains(arg.ToLower());
        }

        public static void HandleArgs(string[] args)
        {
            if (args.Length == 0){
                WriteToLogFile("No parameter provided, exiting..");
                Console.WriteLine("No parameter provided, exiting..");
                Environment.Exit(1);
            }
            if (args.Length < 1 || args.Length > 2) { WriteToLogFile("No parameter provided, exiting.."); Environment.Exit(0); }
            if (!IsValidArg(args[0]))
            {
                WriteToLogFile("Invalid argument provided for cli execution, expected 'download' or 'help'");
                Environment.Exit(1);
            }

            else
            {
                switch (args[0])
                {
                    case "download":
                        if (args.Length != 2)
                        {
                            Console.WriteLine("Invalid download command, expected wsb.exe download <argument>");
                            WriteToLogFile("Invalid download command, expected wsb.exe download <argument>");
                            Environment.Exit(1);
                        }

                        HandleDownload(args[1]);
                        Environment.Exit(0);
                        break; // the dotnet compiler is stupid ---> why must i define break; when Environment.Exit(0); closes the application.

                    case "help":
                        HandleHelp();
                        Environment.Exit(0);
                        break; // the dotnet compiler is stupid ---> why must i define break; when Environment.Exit(0); closes the application.

                     default:
                        HandleHelp();
                        Environment.Exit(0);
                        break; // the dotnet compiler is stupid ---> why must i define break; when Environment.Exit(0); closes the application.
                }
            }
        }


        static void HandleDownload(string downloadName)
        {
            #region OLD CODE FOR REFERENCE
            //Dictionary<String, TimedFunctionDelegate> string_function_mapping = new Dictionary<string, TimedFunctionDelegate>
            //{
            //    { "amd", new TimedFunctionDelegate(new Utils(download_dir).AMD) },
            //    { "roblox", new TimedFunctionDelegate(new Utils(download_dir).Roblox) },
            //    { "vivaldi", new TimedFunctionDelegate(new Utils(download_dir).Vivaldi) },
            //};
            //try
            //{
            //    TimedFunctionDelegate choice = string_function_mapping[downloadName];

            //    choice.Invoke(timer);
            //}
            //catch (KeyNotFoundException)
            //{ HandleHelp(); }
            //catch (Exception ex)
            //{
            //    Console.WriteLine("Please write code to handle the following error:");
            //    Console.WriteLine("Type:", ex.GetType().Name);
            //    Console.WriteLine("Error:");
            //    Console.WriteLine(ex.Message);

            //}
            #endregion

            string download_dir = GetDownloadDir();
            Timer timer = CreateTimer();
            switch (downloadName.ToLower())
            {
                case "amd":
                    try
                    {
                        new TimedFunctionDelegate(new Utils(download_dir).AMD).Invoke(timer);
                        break;
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine("Please write code to handle the following error:");
                        Console.WriteLine("Type:", ex.GetType().Name);
                        Console.WriteLine("Error:");
                        Console.WriteLine(ex.Message);
                        break;
                    }
                    
                case "roblox":
                    try
                    {
                        new TimedFunctionDelegate(new Utils(download_dir).Roblox).Invoke(timer);
                        break;
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine("Please write code to handle the following error:");
                        Console.WriteLine("Type:", ex.GetType().Name);
                        Console.WriteLine("Error:");
                        Console.WriteLine(ex.Message);
                        break;
                    }
           
                case "vivaldi":
                    try
                    {
                        new TimedFunctionDelegate(new Utils(download_dir).Vivaldi).Invoke(timer);
                        break;
                    }
                    catch (Exception ex)
                    {
                        Console.WriteLine("Please write code to handle the following error:");
                        Console.WriteLine("Type:", ex.GetType().Name);
                        Console.WriteLine("Error:");
                        Console.WriteLine(ex.Message);
                        break;
                    }
         
            }
            endTime = DateTime.Now;
            // Typecasting is used here because TotalMilliseconds can slightly vary and an int value isnt valid for this usecase.
            //https://www.codeproject.com/Articles/61964/Performance-Tests-Precise-Run-Time-Measurements-wi
            double raw_runtime = ((TimeSpan)(endTime - startTime)).TotalSeconds;

            // F2 functions identically to python's string interpolation;
            // - F indicates a float point value
            // - 2 indicates the offset/output seperator (number of digits allowed after a decimal)
            var runtime = $"{raw_runtime.ToString("F2")}";
            Console.Write("Downloaded in " + runtime + "s");
        }

        static void HandleHelp()
        {
            Console.WriteLine(helpMessage);
            Console.WriteLine("\n\nPlease press any key to exit...");
            Console.ReadKey();
            Environment.Exit(0);
        }


        #endregion 
        
        #region Browser Functions
        public IWebDriver driver { get; private set; }
        public int pid { get; private set; }
        public Utils(string download_directory)
        {
            // https://www.lifewire.com/firefox-about-config-entry-browser-445707
            // The value of browser.download.folderList can be set to 0, 1, or 2.
            // When set to 0, Firefox saves all downloaded files to the user's desktop. 
            // When set to 1, these downloads go to the Downloads folder. 
            // When set to 2, the location specified for the most recent download is used again. 
            try
            {
                FirefoxDriverService driverService = FirefoxDriverService.CreateDefaultService();
                driverService.HideCommandPromptWindow = true;
                //driverService.SuppressInitialDiagnosticInformation = true;
                FirefoxOptions options = new FirefoxOptions();
                // Since this is going be a headless application, this isn't really needed but it's personal preference.
                options.AddArguments(new string[] { "--headless", "--start-maximized",  "log-level=3" });
                options.SetPreference("browser.download.folderList", 2);
                options.SetPreference("browser.download.dir", download_directory);
                options.SetPreference("browser.download.manager.showWhenStarting", "false");
                options.SetPreference("browser.helperApps.neverAsk.saveToDisk", "application/octet-stream, application/vnd.microsoft.portable-executable");

                driver = new FirefoxDriver(driverService, options);
                driver.Manage().Timeouts().PageLoad = TimeSpan.FromSeconds(300); // 5 minutes in seconds
                pid = driverService.ProcessId;
                
            }
            catch { Environment.Exit(1); }
        }
        public void Dispose()
        {
            driver.Quit();
            Process process = Process.GetProcessById(pid);
            process.Dispose();
        }
        #endregion BrowserUtil

        #region Download Helpers

        public static string GetDownloadDir()
        {
            string full_path = AppContext.BaseDirectory;
            //string full_path = Assembly.GetExecutingAssembly().Location;
            var cd = Path.GetDirectoryName(full_path);
            if (cd != null)
            {
                string download_dir = String.Join("\\", [cd, "applications"]);
                try // Try to silently create the applications directory
                {
                    if (!Directory.Exists(download_dir))
                    {
                        Directory.CreateDirectory(download_dir);
                    }
                }
                catch { } // If for some reason the directory isn't created, it will be created in a seperate function.

                return download_dir; // Will modify if any logical flaws are found in the first two lines of this function
            }
            Console.WriteLine("Unable to create application subdirectory from within application base directory, using desktop instead");
            WriteToLogFile("Unable to create application subdirectory from within application base directory, using desktop instead");
            return "C:\\Users\\" + Environment.UserName + "\\Desktop\\applications";
        }

        public static List<int> GetExistingFileCount(string download_dir)
        {
            List<string> verifiedExtensions = [".exe", ".msi", ".zip"];
            try
            {
                if (!Directory.Exists(download_dir))
                {
                    Directory.CreateDirectory(download_dir);
                    return [0, 0];
                }

                // Parses all files in the download dir, and extracts only those containing a verified extension through the use of a lambda expression, then returns the number of elements found. 
                var files = Directory.GetFiles(download_dir);
                var downloadedApplications = files.Where(filepath => verifiedExtensions.Contains(Path.GetExtension(filepath))).ToList().Count;
                var partialFiles = files.Where(filepath => Path.GetExtension(filepath) == ".part").ToList().Count;
                
                // Debugging
                //Console.WriteLine(downloadedApplications);
                //Console.WriteLine(partialFiles);

                return [downloadedApplications, partialFiles];


            }
            // -1 would cause a false positive if IsDownloaded() is called with a 0 value for existing_file_count and current_file_count returns -1 
            catch (Exception ex){ Console.WriteLine(ex.Message); return [0, 0]; }
        }



        public static bool IsDownloaded(string download_dir, int existing_file_count)
        {
            List<int> file_count = GetExistingFileCount(download_dir);
            int current_file_count = file_count[0];
            int partial_file_count = file_count[1];
            return current_file_count == (existing_file_count+1) && partial_file_count == 0; // Ensure no temp files are present and an additional valid file is detected.
        }

        public static Timer CreateTimer()
        {
            Timer timer = new Timer();
            timer.Interval = 120000;
            return timer;
        }
        // Since c# doesnt allow void functions to be used as parameters, a delegate must be created, and honestly its easier to use delegates when dealing with functions as parameters.
        public delegate bool TimedFunctionDelegate(Timer timer);
        //public delegate void FunctionDelegate(Func<bool> function);
        #endregion

        #region Download Functions
        public bool AMD(Timer timer)
        {
            string download_dir = GetDownloadDir();
            int existing_file_count = GetExistingFileCount(download_dir)[0];
            try
            {
                //Console.WriteLine("Downloading AMD Auto Detect, please wait..");
                
                driver.Navigate().GoToUrl("https://www.amd.com/en/support/download/drivers.html/en/support/download/drivers.html");
                
                //Thread.Sleep(2000); // Accounts for the cookies popup
                //driver.Manage().Window.Size = new System.Drawing.Size(1920, 1080);
                try { driver.FindElement(By.CssSelector("#onetrust-accept-btn-handler")).Click(); Thread.Sleep(1000); }
                catch { }
                driver.FindElement(By.CssSelector("#button-8dbca2589a > span")).Click();
                while (timer.Enabled && !IsDownloaded(download_dir, existing_file_count))
                {
                    if (IsDownloaded(download_dir, existing_file_count))
                    {
                        timer.Stop();
                        driver.Dispose();
                        break;
                    }
                }

                if (!timer.Enabled && !IsDownloaded(download_dir, existing_file_count)) { Console.WriteLine("Timed out downloading..."); driver.Dispose(); return false; }
                driver.Dispose();
                return true;
            }
            catch (Exception e) { Console.WriteLine(e.ToString()); return false; }
        }

        public bool Roblox(Timer timer)
        {
            string download_dir = GetDownloadDir();
            int existing_file_count = GetExistingFileCount(download_dir)[0];
            try
            {
                //Console.WriteLine("Downloading Roblox Launcher, please wait..");
                driver.Navigate().GoToUrl("https://www.roblox.com/download");
                //Thread.Sleep(2000);
                driver.FindElement(By.CssSelector(".download-button")).Click();
                while (timer.Enabled && !IsDownloaded(download_dir, existing_file_count))
                {
                    if (IsDownloaded(download_dir, existing_file_count))
                    {
                        timer.Stop();
                        driver.Dispose();
                        break;
                    }
                }

                if (!timer.Enabled && !IsDownloaded(download_dir, existing_file_count)) { Console.WriteLine("Timed out downloading..."); driver.Dispose(); return false; }
                driver.Dispose();
                return true;
            }
            catch (Exception e) { Console.WriteLine(e.ToString()); return false; }
        }

        public bool Vivaldi(Timer timer)
        {
            timer.Start();
            string download_dir = GetDownloadDir();
            int existing_file_count = GetExistingFileCount(download_dir)[0];
            try
            {
                //Console.WriteLine("Downloading Vivaldi Browser, please wait..");
                driver.Navigate().GoToUrl("https://vivaldi.com/download/");
                //Thread.Sleep(2000); // Calculate the download speed then divide it in half then calculate the total request size of the download page.
                driver.FindElement(By.CssSelector(".download-button")).Click();
                while (timer.Enabled && !IsDownloaded(download_dir, existing_file_count))
                {
                    // IDEA:
                    // Calculate download speed here, divide it by 1.3 (to handle lag spikes), then calculate the application file size + request page size;
                    // Finally divide the download size by the download speed this will provide a somewhat accurate sleep function (in theory)
                    // 
                    // SIDE NOTE:
                    // There's more factors to the download, such as disk speed (both read and write), cpu clock speed, and free ram amount + ram speed; so this might not be feasible.
                    Thread.Sleep(500);
                    if (IsDownloaded(download_dir, existing_file_count))
                    {
                        timer.Stop();
                        driver.Dispose();
                        break;
                    }

                }
                if (!timer.Enabled && !IsDownloaded(download_dir, existing_file_count)) { Console.WriteLine("Timed out downloading..."); driver.Dispose();  return false; }
                driver.Dispose();
                return true;
            }
            catch (Exception e) { Console.WriteLine(e.ToString()); return false; }
        }

        #endregion

        #region Log File Functions

        static bool LogFileExists()
        {
            try
            {
                return Directory.GetFiles(Environment.CurrentDirectory).Contains("runtime.log");
            }
            catch (Exception ex)
            {
                Console.Write("Please write code to handle the following error");
                Console.WriteLine(ex.GetType().Name);
                Console.WriteLine(ex.Message);
                return false;
            }
        }
        static bool CreateLogFile()
        {
            try
            {
                using (var file = File.Create("runtime.log"))
                {
                    return true;
                }
            }
            catch (Exception ex)
            {
                switch (ex)
                {
                    case UnauthorizedAccessException: Console.WriteLine(ex.Message); return false;
                    case ArgumentException: Console.WriteLine(ex.Message); return false;
                    case PathTooLongException: Console.WriteLine(ex.Message); return false;
                    case DirectoryNotFoundException: Console.WriteLine(ex.Message); return false;
                    case IOException: Console.WriteLine(ex.Message); return false;
                    case NotSupportedException: Console.WriteLine(ex.Message); return false;
                    default: return false;
                }
            }
        }
        
        public static void WriteToLogFile(string contents)
        {
            string time = "";
            try { time = DateTime.Now.ToString() + " -> "; }
            catch { Environment.Exit(0); }
            string sanitized_contents = time + contents + "\n";

            #region Check if log file exists prior to writing
            if (!LogFileExists()){

                bool found = false;
                int attempts = 0;
                while (attempts < 3)
                {
                    bool result = CreateLogFile();
                    attempts++;
                    if (result)
                    {
                        found = true;
                        break;
                    }
                    

                    Console.WriteLine("Unable to create runtime log, trying again.");
                    continue;
                    
                }

                if (!found){
                    Console.WriteLine("Unable to write to runtime log, as the file couldn't be created");
                    return;
                }
            }
            #endregion

            #region Writing to the runtime log
            try
            {
                using (var file = File.Open("runtime.log", FileMode.Append, FileAccess.Write))
                {
                    file.Write(Encoding.UTF8.GetBytes(sanitized_contents));
                }
            }
            catch (Exception ex)
            
            {
                switch (ex)
                {
                    case UnauthorizedAccessException: Console.WriteLine(ex.Message); break;
                    case ArgumentException: Console.WriteLine(ex.Message); break;
                    case PathTooLongException: Console.WriteLine(ex.Message); break;
                    case DirectoryNotFoundException: Console.WriteLine(ex.Message); break;
                    case IOException: Console.WriteLine(ex.Message); break;
                    case NotSupportedException: Console.WriteLine(ex.Message); break;
                    default: break;
                }
            }
            #endregion


        }

        #endregion
    }

}