use std::collections::{HashMap, HashSet};

use osmpbf::{DenseNodeInfo, DenseTagIter, Element, ElementReader, Info, TagIter};

use crate::infos;
use crate::infos::GatheredInfos;

pub fn process_history(
    history_file_path: &str,
    tag_list: HashSet<&str>,
) -> HashMap<i64, GatheredInfos> {
    let mut elements_info: HashMap<i64, GatheredInfos> = HashMap::new();
    // 
    let reader_result = ElementReader::from_path(history_file_path);

    let reader = match reader_result {
        Ok(reader) => reader,
        Err(e) => panic!("Problem with Reader creation: {:?}", e),
    };

    match reader.for_each(|element| match element {
        Element::DenseNode(node) => {
            if filter_dense_tags(&tag_list, node.tags()) {
                let new_info = create_dense_info(&node.info().unwrap());
                match elements_info.get_mut(&node.id()) {
                    Some(infos) => infos.add_info(new_info),
                    None => {
                        let mut new_infos: GatheredInfos = GatheredInfos::new();
                        new_infos.add_info(new_info);
                        elements_info.insert(node.id(), new_infos);
                    }
                }
            }
        }
        Element::Node(node) => {
            if filter_tags(&tag_list, node.tags()) {
                let new_info = create_element_info(&node.info());
                match elements_info.get_mut(&node.id()) {
                    Some(infos) => infos.add_info(new_info),
                    None => {
                        let mut new_infos: GatheredInfos = GatheredInfos::new();
                        new_infos.add_info(new_info);
                        elements_info.insert(node.id(), new_infos);
                    }
                }
            }
        }
        Element::Way(way) => {
            if filter_tags(&tag_list, way.tags()) {
                let new_info = create_element_info(&way.info());
                match elements_info.get_mut(&way.id()) {
                    Some(infos) => infos.add_info(new_info),
                    None => {
                        let mut new_infos: GatheredInfos = GatheredInfos::new();
                        new_infos.add_info(new_info);
                        elements_info.insert(way.id(), new_infos);
                    }
                }
            }
        }
        Element::Relation(relation) => {
            if filter_tags(&tag_list, relation.tags()) {
                let new_info = create_element_info(&relation.info());
                match elements_info.get_mut(&-relation.id()) {
                    Some(infos) => infos.add_info(new_info),
                    None => {
                        let mut new_infos: GatheredInfos = GatheredInfos::new();
                        new_infos.add_info(new_info);
                        elements_info.insert(-relation.id(), new_infos);
                    }
                }
            }
        }
    }) {
        Ok(_) => {
            println!(
                "History processing finished : {} elements processed",
                elements_info.len()
            );
        }
        Err(e) => {
            panic!("Failed to process history : {}", e);
        }
    }
    elements_info
}

fn create_element_info(info: &Info) -> infos::Info {
    infos::Info {
        version: info.version().unwrap_or(-1),
        changeset: info.changeset().unwrap_or(-1),
        uid: info.uid().unwrap_or(-1),
        timestamp: info.milli_timestamp().unwrap_or(-1),
    }
}

fn create_dense_info(info: &DenseNodeInfo) -> infos::Info {
    infos::Info {
        version: info.version(),
        changeset: info.changeset(),
        uid: info.uid(),
        timestamp: info.milli_timestamp(),
    }
}

fn filter_dense_tags(tag_list: &HashSet<&str>, mut tags: DenseTagIter) -> bool {
    tags.any(|t| tag_list.contains(&t.0))
}

fn filter_tags(tag_list: &HashSet<&str>, mut tags: TagIter) -> bool {
    tags.any(|t| tag_list.contains(&t.0))
}
