extern crate zip;

use std::io::prelude::*;
use zip::CompressionMethod;
use zip::write::FileOptions;
use std::io::Cursor;

// This test asserts that after creating a zip file, then reading its contents back out,
// the extracted data will *always* be exactly the same as the original data.
#[test]
fn end_to_end() {
    let file = &mut Cursor::new(Vec::new());

    write_to_zip_file(file).expect("file written");

    check_contents(file, ENTRY_NAME);
}

// This test asserts that after copying a `ZipFile` to a new `ZipWriter`, then reading its
// contents back out, the extracted data will *always* be exactly the same as the original data.
#[test]
fn copy() {
    let src_file = &mut Cursor::new(Vec::new());
    write_to_zip_file(src_file).expect("file written");

    let mut tgt_file = &mut Cursor::new(Vec::new());

    {
        let mut archive = zip::ZipArchive::new(src_file).unwrap();
        let mut zip = zip::ZipWriter::new(&mut tgt_file);

        {
            let file = archive.by_name(ENTRY_NAME).expect("file found");
            zip.copy_file(file).unwrap();
        }

        {
            let file = archive.by_name(ENTRY_NAME).expect("file found");
            zip.copy_file_rename(file, COPY_ENTRY_NAME).unwrap();
        }
    }

    check_contents(tgt_file, ENTRY_NAME);
    check_contents(tgt_file, COPY_ENTRY_NAME);
}

fn write_to_zip_file(file: &mut Cursor<Vec<u8>>) -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipWriter::new(file);

    zip.add_directory("test/", FileOptions::default())?;

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);
    zip.start_file("test/☃.txt", options)?;
    zip.write_all(b"Hello, World!\n")?;

    zip.start_file(ENTRY_NAME, FileOptions::default())?;
    zip.write_all(LOREM_IPSUM)?;

    zip.finish()?;
    Ok(())
}

fn read_zip_file(zip_file: &mut Cursor<Vec<u8>>, name: &str) -> zip::result::ZipResult<String> {
    let mut archive = zip::ZipArchive::new(zip_file).unwrap();

    let mut file = archive.by_name(name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

fn check_contents(zip_file: &mut Cursor<Vec<u8>>, name: &str) {
    let file_contents: String = read_zip_file(zip_file, name).unwrap();
    assert!(file_contents.as_bytes() == LOREM_IPSUM);
}

const LOREM_IPSUM : &'static [u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. In tellus elit, tristique vitae mattis egestas, ultricies vitae risus. Quisque sit amet quam ut urna aliquet
molestie. Proin blandit ornare dui, a tempor nisl accumsan in. Praesent a consequat felis. Morbi metus diam, auctor in auctor vel, feugiat id odio. Curabitur ex ex,
dictum quis auctor quis, suscipit id lorem. Aliquam vestibulum dolor nec enim vehicula, porta tristique augue tincidunt. Vivamus ut gravida est. Sed pellentesque, dolor
vitae tristique consectetur, neque lectus pulvinar dui, sed feugiat purus diam id lectus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per
inceptos himenaeos. Maecenas feugiat velit in ex ultrices scelerisque id id neque.
";

const ENTRY_NAME : &str = "test/lorem_ipsum.txt";

const COPY_ENTRY_NAME : &str = "test/lorem_ipsum_renamed.txt";
