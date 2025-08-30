using System;
using System.Threading;
using System.Collections.Generic;
using System.Linq;
using CLI;

namespace DU_CLI
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Utils.HandleArgs(args);
            //Thread.Sleep(1000000);
            //string download_dir = Utils.GetDownloadDir();
            //Console.WriteLine(download_dir);
            //Utils utils = new Utils(download_dir);
        }
    }
}
