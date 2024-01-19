use std::{io, fs};

use image::{GenericImageView, RgbImage};

struct ImgData {
    r: u128,
    g: u128,
    b: u128,
    path: String
}



fn main() {
    // /home/jost/project/image-sort
    println!("Введите путь к корневой папке: ");
    let mut path = String::new();

    // Считываем строку от пользователя
    io::stdin().read_line(&mut path)
        .expect("Не удалось прочитать строку");

    println!("Введите путь к целевой папке: ");
    let mut path2 = String::new();

    // Считываем строку от пользователя
    io::stdin().read_line(&mut path2)
        .expect("Не удалось прочитать строку");


    let mut r: Vec<ImgData> = Vec::new();
    let mut g: Vec<ImgData> = Vec::new();
    let mut b: Vec<ImgData> = Vec::new();
    let file_count: usize = 0;

    if let Ok(entries) = fs::read_dir(path.trim()) {
        let file_count = entries.filter(|entry| entry.is_ok() && entry.as_ref().unwrap().file_type().map(|ft| ft.is_file()).unwrap_or(false)).count();
        println!("Найдено {} файлов", file_count);
    }

    if let Ok(entries) = fs::read_dir(path.trim()) {
        let mut index = 0;
        for entry in entries {
            if let Ok(entry) = entry {
                if entry.path().is_file() {
                    if let Some(file_path) = entry.path().to_str(){
                        let mut image = read_img(file_path);
                        if image.r > image.g && image.r > image.b {
                            r.push(image)
                        } else if image.g > image.r && image.g > image.b {
                            g.push(image)
                        } else if image.b > image.r && image.b > image.g {
                            b.push(image)
                        }
                    }
                }
                else {
                    println!("{} is skiped", entry.path().display());
                }
                println!("Прочитанно \t{}/{}\r", index, file_count);
            }
        }
        let mut index = 0;
        let new_len = r.len()+g.len()+b.len();

        r.sort_by_key(|img_d| img_d.r);
        g.sort_by_key(|img_d| img_d.g);
        b.sort_by_key(|img_d| img_d.b);



        copy(new_len, r, path.clone(), &path2.clone(), &mut index);
        copy(new_len, g, path.clone(), &path2.clone(), &mut index);
        copy(new_len, b, path.clone(), &path2.clone(), &mut index);

        println!("Оно работает!");
    } else {
        eprintln!("Не удалось прочитать содержимое папки.");
    }
}

fn read_img(path: &str) -> ImgData{
    let mut image = ImgData {
        r: 0,
        g: 0,
        b: 0,
        path: path.to_string()
    };
    if let Ok(img) = image::open(path.clone()) {
        for y in 0..img.height() {
            for x in 0..img.width() {
                // Получаем цвет пикселя из оригинала
                let pixel = img.get_pixel(x, y);
                
                image.r+=pixel[0] as u128;
                image.g+=pixel[1] as u128;
                image.b+=pixel[2] as u128;
            }
        }
        image
    } else {
        println!("Error read image {}", path);
        image
    }
}

fn copy(len: usize, data: Vec<ImgData>, path: String, mut path2: &String, mut index: &i32) {
    let mut index = *index;
    for i in data.iter() {
        let source_path = i.path.as_str();
        let mut new_path = path2.clone().trim().to_string();
        new_path.push_str((index.to_string()+".jpg").as_str());

        let destination_path = new_path.as_str();

        
        if let Ok(content) = fs::read(source_path) {
            // Запись содержимого в целевой файл
            if let Ok(()) = fs::write(destination_path, content) {
                index+=1;
            }
        }
        println!("Выполнено {}/{}\r", index, len);
    }
}