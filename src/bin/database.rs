use yogamat_wasm::Asana;
use rusqlite::Connection;
use std::{collections::HashMap, fs::File, io::{Read, Write}};
use yogamat_wasm::{skeleton::Joint, AsanaData};

fn main() {
    serialize_db();
    let data: AsanaData = deserialize_db();
    println!("{:#?}", data.asanas);
}

fn deserialize_db() -> AsanaData {
    let mut db_file = File::options()
        .read(true)
        .open("out_db.new")
        .expect("file couldn't be read");
    let mut db = Vec::new();
    let _bytes_read = db_file.read_to_end(&mut db).expect("error reading file");
    bincode::decode_from_slice(&db, bincode::config::legacy())
        .unwrap()
        .0
}

#[cfg(not(target_arch = "wasm32"))]
fn get_asanas_from_db() -> Vec<Asana> {
    let path = "./yogamatdb.sql";
    let db = Connection::open(path).expect("couldn't open database");
    let sql = r#"
SELECT a.poseId, a.asanaID, b.sanskritName, b.englishName, b.userNotes
FROM pose a, asana b
WHERE a.asanaID = b.asanaID;
"#;
    let mut stmt = db.prepare(sql).expect("trouble preparing statement");
    let response = stmt
        .query_map([], |row| {
            Ok(Asana {
                pose_id: row.get(0).expect("poseId"),
                asana_id: row.get(1).expect("asanaID"),
                sanskrit: row.get(2).expect("sanskritName"),
                english: row.get(3).expect("englishName"),
                notes: row.get(4).expect("userNotes"),
            })
        })
        .expect("bad");
    response
        .filter_map(|result| result.ok())
        .collect::<Vec<Asana>>()
}

#[cfg(not(target_arch = "wasm32"))]
fn serialize_db() {
    let asanas = get_asanas_from_db();
    let mut data = AsanaData {
        asanas,
        poses: HashMap::new(),
    };

    let path = "./yogamatdb.sql";
    let db = Connection::open(path).expect("couldn't open database");

    for asana in data.asanas.iter() {
        let sql = format!("select * from joint where poseID = {};", asana.pose_id);
        let mut stmt = db.prepare(&sql).expect("trouble preparing statement");
        let response = stmt
            .query_map([], |row| {
                Ok(Joint {
                    joint_id: row.get(0).expect("so may results"),
                    pose_id: row.get(1).expect("so may results"),
                    up_x: row.get(2).expect("so may results"),
                    up_y: row.get(3).expect("so may results"),
                    up_z: row.get(4).expect("so may results"),
                    forward_x: row.get(5).expect("so may results"),
                    forward_y: row.get(6).expect("so may results"),
                    forward_z: row.get(7).expect("so may results"),
                    origin_x: row.get(8).expect("so may results"),
                    origin_y: row.get(9).expect("so may results"),
                    origin_z: row.get(10).expect("so may results"),
                })
            })
            .expect("bad");
        let joints = response
            .filter_map(|result| result.ok())
            .collect::<Vec<Joint>>();
        let already = data.poses.insert(asana.pose_id, joints);
        assert!(already.is_none());
    }
    let config = bincode::config::standard()
        .with_little_endian()
        .with_fixed_int_encoding();

    let encoded: Vec<u8> = bincode::encode_to_vec(&data, config).unwrap();
    let mut out_file = File::options()
        .write(true)
        .create(true)
        .truncate(true)
        .open("out_db.new")
        .expect("file couldn't be created");
    let success = out_file.write_all(&encoded);
    match success {
        Ok(_) => println!("encoded db written to out_db"),
        Err(_) => panic!("encoded db file write failed"),
    }
}

