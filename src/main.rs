use audiotags::Tag;
use fs_extra::{dir, file};
use std::fs;
use std::io;
use walkdir::{DirEntry, WalkDir};
use colored::Colorize;

fn main() {
    println!("{}","Muffin".bright_purple());
    println!("Please enter the directory path you would like to modify:");
    let mut dir_input = String::new();
    io::stdin()
        .read_line(&mut dir_input)
        .expect("Failed to read line");
    // println!("{}/sex/one", dir_input.trim()); // print to debug
    //    // mv_files();

       itr_dir(&dir_input);
      

}

fn ukn_fn(dir_input: &String,f_names:Vec<std::string::String>) {
   for fname in f_names.iter() {
        // Construct the full path for each file
        let path = format!(
            "{}{}", // Assuming the files are directly under the directory
            dir_input.trim(),
            fname.trim()
        );

        // Read the tag information (metadata) from the file at the path
        // creates a new instance of Tag
        let tag = Tag::new()
            .read_from_path(&path)
            .unwrap(); // You can handle the error more gracefully instead of unwrap()
        
        // Extract artist and album from the file's metadata
        let artist = tag.artist().unwrap();
        let album = tag.album().unwrap();

        // Print the filename and extracted metadata (artist and album)
       //  println!("{}",artist);
        create_dir(artist, album.title, &dir_input);
        mv_files(&dir_input, artist, album.title, f_names.clone());
    }
    }
 

fn create_dir(artist: &str, album: &str, dir_input: &String) {
    // gotta use format macro cuz rust doesn't have string interpolation thingy
    let path = format! {"{}{}/{}/",dir_input.trim(), artist, album};
    println!("{}", path);
    fs::create_dir_all(path);
}

fn mv_files(dir_input: &String, artist: &str, album: &str,f_names:Vec<std::string::String>) {
   /*
    let mut options = dir::CopyOptions::new();
    let mut from_paths = Vec::new();
    for fname in f_names.iter() {
    from_paths.push(format!(
        "{}{}/{}",
        dir_input, artist, album,
    ));
    }
    for fname in f_names.iter() {


    match fs_extra::copy_items(&from_paths,"{dir_input}/{fname}", &options) {
        Ok(_) => println!("Copied"),
        Err(e) => println!("Error: {}", e),
    }

    }
   */   
    let mut options = file::CopyOptions::new();
    for fname in f_names.iter() {
        let source_path = format!("{}{}", dir_input.trim(),fname.trim());
        let dest_path = format!("{}{}/{}/{}", dir_input.trim(), artist.trim(), album.trim(), fname.trim());
        match file::move_file(&source_path, &dest_path, &options) {
            Ok(_) => println!("Successfully moved: {}", fname),
            Err(e) => eprintln!("Error moving file {}: {}", fname, e),
        }
    }

}

fn itr_dir(dir_input: &String)-> Vec<std::string::String> {
    /*
    let file_name = fs::read_dir("{}", dir_input.trim()).unwrap();
     for file in file_name {
            println!("{}", file.unwrap());
    }
        */
    let mut f_names = Vec::new();

    for file in WalkDir::new(dir_input.trim())
        .max_depth(1)
        .into_iter()
        .filter_map(|file| file.ok())
    {
        if file.metadata().unwrap().is_file() {
            let f_name = file.file_name().to_string_lossy().into_owned();

            // println!("{}", file.metadata().unwrap().is_file());
            println!("{}", f_name);
            f_names.push(f_name);
            // https://stackoverflow.com/a/31667995
        }
    }
    ukn_fn(&dir_input, f_names.clone());
    return f_names;
}

