use std::env;
use std::path::PathBuf;

fn main() {
    let mut args = env::args();

    let _program_name = args.next();
    let export_path = args.next().map(PathBuf::from).unwrap();
    let import_path = args.next().map(PathBuf::from).unwrap();
    assert!(args.next().is_none());
    println!("exporting {:?} into {:?}", export_path, import_path);

    let export_db = sled::Db::open(export_path).unwrap();
    let import_db = sled::Db::open(import_path).unwrap();

    import_db.import(export_db.export());
    eprintln!("successful export > import");
}
