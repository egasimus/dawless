use binread::{BinRead, io::{Cursor, Seek, SeekFrom}};

fn main() {
    //let filename = "flashfloppy-images/Unformatted/HFE/double_density.hfe";
    //let filename = "flashfloppy-images/Unformatted/HFE/double_density_360rpm.hfe";
    //let filename = "flashfloppy-images/Unformatted/HFE/high_density.hfe";
    //let filename = "flashfloppy-images/Unformatted/HFE/high_density_360rpm.hfe";
    //let filename = "./data/EMPTY.hfe";
    let filename = "./data/DISK0.hfe";
    let mut f = std::fs::File::open(&filename).expect("file not found");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    let mut cursor = Cursor::new(buffer);

    let header = hfe::Header::read(&mut cursor).expect("header read failed");
    cursor.seek(SeekFrom::Start(0x200u64 * (header.track_list_offset as u64))).unwrap();
    println!("\n{:?}", &header);

    let mut track_info: Vec<hfe::TrackInfo> = vec![];
    for _ in 0..header.tracks {
        track_info.push(hfe::TrackInfo::read(&mut cursor).expect("track read failed"));
    }
    println!("\n{:?}", &track_info);

    let mut track_data: Vec<hfe::Track> = vec![];
    for track_index in 0..header.tracks {
        let info = track_info.get(track_index as usize).unwrap();
        let mut blocks: Vec<hfe::TrackBlock> = vec![];
        let offset = 0x200u64 * info.offset as u64;
        cursor.seek(SeekFrom::Start(offset)).unwrap();
        loop {
            if cursor.position() >= offset + info.length as u64 {
                break
            }
            let block = hfe::TrackBlock::read(&mut cursor).expect("block read failed");
            println!("{}", &block);
            blocks.push(block);
        }
        track_data.push(hfe::Track { info: info.clone(), blocks })
    }
}

