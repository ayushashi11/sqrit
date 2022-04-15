#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::{path::{PathBuf, Path}, fs::{File, create_dir_all}, io::{Read, Write}};
use rocket::response::{NamedFile, content};
use uuid::Uuid;
use rocket::request::{Form, FromForm};
use rocket::http::ContentType;

#[derive(FromForm)]
struct Input{
    text: String
}

#[post("/input", data = "<input>")]
fn new(input: Form<Input>) -> Result<String, String> { 
    let id = Uuid::new_v4();
    let sid = format!("{}", id.to_simple());
    println!("{}", sid);
    let path = PathBuf::new().join("files").join(&sid);
    create_dir_all(path.clone()).ok().expect("unable to create directory");
    let tpath = path.join("text.txt");
    let mut f = File::create(tpath).ok().expect("unable to create file");
    f.write_all(input.0.text.as_bytes()).ok();
    let qr = qr_code::QrCode::new(sid.clone()).unwrap();
    let bmp = qr.to_bmp();
    let qrpath = path.join("qr.bmp");
    bmp.write(File::create(qrpath).unwrap()).unwrap();
    Ok(sid) 
}

#[get("/")]
fn index() -> content::Content<String> {
    content::Content(ContentType::HTML, "
    <form action=\"/input\" method=\"POST\">
    <input name=\"text\"/>
    <input type=\"submit\"/>
    </form>
    ".to_string())
}

#[get("/url/<name..>")]
fn url(name: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(name).join("text.txt")).ok()
}

#[get("/qrs/<name..>")]
fn qrs(name: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(name).join("qr.bmp")).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![index, url, new, qrs]).launch();
}
