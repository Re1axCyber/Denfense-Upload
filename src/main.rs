// 导入必要的库
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::time::SystemTime;
use reqwest::{Client};
#[derive(Debug)]
// 定义一个结构体来封装文件信息
struct FileInfo {
    path: String,
    modified_time: SystemTime,
}
fn print_logo(){
    let logo=r" ______   _______  _______  _______  _        _______  _______               _______  _        _______  _______  ______
(  __  \ (  ____ \(  ____ \(  ____ \( (    /|(  ____ \(  ____ \    |\     /|(  ____ )( \      (  ___  )(  ___  )(  __  \
| (  \  )| (    \/| (    \/| (    \/|  \  ( || (    \/| (    \/    | )   ( || (    )|| (      | (   ) || (   ) || (  \  )
| |   ) || (__    | (__    | (__    |   \ | || (_____ | (__  _____ | |   | || (____)|| |      | |   | || (___) || |   ) |
| |   | ||  __)   |  __)   |  __)   | (\ \) |(_____  )|  __)(_____)| |   | ||  _____)| |      | |   | ||  ___  || |   | |
| |   ) || (      | (      | (      | | \   |      ) || (          | |   | || (      | |      | |   | || (   ) || |   ) |
| (__/  )| (____/\| )      | (____/\| )  \  |/\____) || (____/\    | (___) || )      | (____/\| (___) || )   ( || (__/  )
(______/ (_______/|/       (_______/|/    )_)\_______)(_______/    (_______)|/       (_______/(_______)|/     \|(______/

                 _______          _________          _______  _______     _______  _______  __    _______                _______           ______   _______  _______
                (  ___  )|\     /|\__   __/|\     /|(  ___  )(  ____ )   (  ____ )(  ____ \/  \  (  ___  )|\     /|     (  ____ \|\     /|(  ___ \ (  ____ \(  ____ )
                | (   ) || )   ( |   ) (   | )   ( || (   ) || (    )| _ | (    )|| (    \/\/) ) | (   ) |( \   / )     | (    \/( \   / )| (   ) )| (    \/| (    )|
                | (___) || |   | |   | |   | (___) || |   | || (____)|(_)| (____)|| (__      | | | (___) | \ (_) /      | |       \ (_) / | (__/ / | (__    | (____)|
                |  ___  || |   | |   | |   |  ___  || |   | ||     __)   |     __)|  __)     | | |  ___  |  ) _ (       | |        \   /  |  __ (  |  __)   |     __)
                | (   ) || |   | |   | |   | (   ) || |   | || (\ (    _ | (\ (   | (        | | | (   ) | / ( ) \      | |         ) (   | (  \ \ | (      | (\ (
                | )   ( || (___) |   | |   | )   ( || (___) || ) \ \__(_)| ) \ \__| (____/\__) (_| )   ( |( /   \ )     | (____/\   | |   | )___) )| (____/\| ) \ \__
                |/     \|(_______)   )_(   |/     \|(_______)|/   \__/   |/   \__/(_______/\____/|/     \||/     \|_____(_______/   \_/   |/ \___/ (_______/|/   \__/
                                                                                                                  (_____)                                            ";
    println!("{}",logo)
}
impl FileInfo {
    // 构造函数,用于初始化文件信息
    fn new(path: &str) -> Self {
        let metadata = fs::metadata(path).expect("无法获取文件元数据");
        let modified_time = metadata.modified().expect("无法获取文件修改时间");
        FileInfo {
            path: path.to_string(),
            modified_time,
        }
    }
}

// 主函数
#[tokio::main]
async fn main() {
    print_logo();
    // 获取当前目录路径
    let current_dir = std::env::current_dir().unwrap();

    // 读取当前目录下的所有文件和子目录
    let entries = fs::read_dir(&current_dir).unwrap();

    // 创建一个HashMap来存储文件信息
    let mut file_infos: HashMap<String, FileInfo> = HashMap::new();

    // 遍历所有文件,并存储它们的信息
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_info = FileInfo::new(path.to_str().unwrap());
            file_infos.insert(file_info.path.clone(), file_info);
        }
    }
    println!("源文件列表已写入!");

    // 定期检查文件是否被修改或新增
    loop {
        // 遍历所有文件
        //println!("{:?}",file_infos);
        for (path, file_info) in &file_infos {
            let metadata = fs::metadata(path).expect("无法获取文件元数据");
            let modified_time = metadata.modified().expect("无法获取文件修改时间");

            // 检查文件是否被修改
            if modified_time > file_info.modified_time {
                println!("文件 {} 已被修改!", path);
            }

        }

        // 检查是否有新文件被写入
        let new_entries = fs::read_dir(&current_dir).unwrap();
        let mut new_files = Vec::new();
        for entry in new_entries {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_file() && !file_infos.contains_key(path.to_str().unwrap()) {
                new_files.push(path.to_str().unwrap().to_string());
            }
        }
        let req=Client::new();
        // 删除新增的文件
        for new_file in new_files {
            println!("删除新增文件: {}", new_file);
            fs::remove_file(&new_file).expect("无法删除文件");
            let url = format!("http://www.pushplus.plus/send?token=61fae126cd0b4266bd052a1db969d687&title=出现恶意新文件{}&content=已对恶意新文件{}执行删除指令.&template=html",&new_file,&new_file);
            let text=req.get(url).send().await.unwrap().text();
            // println!("{}",text.await.unwrap());
            if text.await.unwrap().contains("请求成功"){
                println!("微信通知成功");
            }
            else{
                println!("微信通知失败");
            }
        }
        // 等待一段时间后再次检查
        tokio::time::sleep(std::time::Duration::from_micros(200)).await;
    }
}