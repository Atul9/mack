use taglib;
use types::{MackError, Track};
use std::path::PathBuf;

pub fn get_track(path: PathBuf) -> Result<Track, MackError> {
    let tl_path = path.clone();
    let file = match tl_path.to_str() {
        Some(file) => file,
        None => {
            let msg =
                format!("Path {:?} contains non-Unicode. This is not supported, so bailing.", path);
            panic!(msg);
        }
    };

    let tag_file = taglib::File::new(file)?;
    Ok(Track {
        path: path,
        tag_file: tag_file,
    })
}

/// We don't intend to print *all* metadata, only ones we might actually try to apply fixes to
pub fn print_track_info(track: &Track) -> () {
    let tags = track.tag_file.tag();

    match tags {
        Ok(tags) => {
            println!("{}:", track.path.display());
            println!("- Album:   {}", tags.album());
            println!("- Artist:  {}", tags.artist());
            println!("- Title:   {}", tags.title());
            println!("- Track #: {}", tags.track());
            println!("- Year:    {}\n", tags.year());
        }
        Err(err) => eprintln!("error printing track info: {}: {:?}", track.path.display(), err),
    }
}
