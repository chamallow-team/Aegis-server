use std::io;
use std::io::Write;
use flate2::Compression;
use flate2::write::GzEncoder;
use log::error;
use rmp_serde::Serializer;
use serde::Serialize;
use tar::{Builder, Header};
use crate::Map;
use crate::save::names::*;

mod names {
    pub(super) const MODULES_DIR: &str = "modules/";
    pub(super) const MAP_NAME: &str = "map.chal";
}

pub fn save(map: &Map) -> io::Result<Vec<u8>> {
    let graph = map.get_map();

    let mut tar = Builder::new(Vec::<u8>::new());


    let mut header = Header::new_gnu();

    // register dirs
    // tar.append_dir(MODULES_DIR, ".")?;

    // map
    {
        let c = match generate_chal_content(graph) {
            Ok(d) => d,
            Err(e) => {
                error!(target: "MapEncoderCompressor", "Cannot compress the datas: {e:#?}");
                return Err(io::Error::new(io::ErrorKind::Other, "Cannot compress the datas"));
            }
        };

        let mut f = std::fs::File::create("/tmp/map.chal").unwrap();
        f.write_all(&c).unwrap();
        drop(f);

        println!("{c:?}");
        dbg!(c.len());
        header.set_path(MAP_NAME)?;
        header.set_size(c.len() as u64);
        header.set_cksum();

        tar.append(&header, c.as_slice())?;
    }

    tar.into_inner().map(|d| compress(&d).unwrap_or(d))
}

fn generate_chal_content(data: impl Serialize) -> Result<Vec<u8>, rmp_serde::encode::Error> {
    let mut buf = Vec::new();
    let mut serializer = Serializer::new(&mut buf);

    data.serialize(&mut serializer)?;

    Ok(buf)
}

fn compress(d: &[u8]) -> io::Result<Vec<u8>> {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());
    e.write_all(d)?;
    e.finish()
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Write;
    use uuid::Uuid;
    use crate::map::{Node, NodeType};

    #[test]
    fn compress_data(){
        let d = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

        let c = super::compress(d);
        assert!(c.is_ok());
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn create_save_file(){
        let mut m = crate::Map::default();
        // build a graph for the example
        {
            let n1 = Uuid::new_v4();
            let n2 = Uuid::new_v4();
            m.get_map_mut().add_node(Node::new(NodeType::Water, (0, 2), n1));
            m.get_map_mut().add_node(Node::new(NodeType::Water, (3, 9), n2));

            m.get_map_mut().add_edge(n1, n2);
        }

        let buf = super::save(&m).unwrap();
        dbg!(buf.len());

        let _ = std::fs::remove_file("/tmp/aegis_test.tar.gz");

        let mut f = File::create("/tmp/aegis_test.tar.gz").unwrap();
        f.write_all(&buf).unwrap();
        drop(f);

        todo!()
    }
}