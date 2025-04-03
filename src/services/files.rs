use std::collections::HashMap;
use std::fs::{read_dir, read_to_string, DirEntry, Metadata};
use std::sync::LazyLock;
use std::time::SystemTime;

use actix_web::{get, web, Responder, Result as ActixResult};
use chrono::{DateTime, Utc};
use serde::Serialize;

fn parse_dir(root: &str) -> FSNode {
    let res = read_dir(root);
    if let Err(e) = res {
        let err = format!("path: {}, error, {:?}", root, e);
        eprintln!("{}", err);
        return FSNode::File(err);
    }

    let res = res.unwrap();
    FSNode::Dir(HashMap::from([(
        root.to_string(),
        res.into_iter()
            // .inspect(|entry| eprintln!("{:?}", entry))
            .map(|entry| match entry {
                Err(e) => FSNode::File(format!("error, {:?}", e)),
                Ok(entry) => {
                    let (file_name, file_type) = (
                        entry.file_name().into_string().unwrap(),
                        entry.file_type().unwrap(),
                    );

                    if file_type.is_file() {
                        FSNode::File(file_name)
                    } else if file_type.is_dir() {
                        parse_dir(&(root.to_string() + "/" + &file_name))
                    } else {
                        FSNode::File("this may be a symlink".into())
                    }
                }
            })
            .collect::<Vec<FSNode>>(),
    )]))
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
enum FSNode {
    #[serde(untagged)]
    File(String),
    #[serde(untagged)]
    Dir(HashMap<String, Vec<FSNode>>),
}

#[get("/files-tree/{root}")]
pub(super) async fn files_tree(root: web::Path<String>) -> ActixResult<impl Responder> {
    // eprintln!("{:?}", parse_dir(root.as_str()));

    Ok(web::Json(parse_dir(root.as_str())))
}

fn inspect_dir(dir: &str) -> HashMap<String, FSMeta> {
    let res = read_dir(dir);
    if let Err(ref e) = res {
        let err = format!("path: {}, error, {:?}", dir, e);
        eprintln!("{}", err);
        panic!("{}", err);
    }

    let res = res.unwrap();
    res.into_iter()
        .filter(|node| node.is_ok())
        .map(|node| {
            let node = node.unwrap();
            (FSMeta::name(&node), FSMeta::from_entry(node))
        })
        .collect()
}

#[derive(Debug, Serialize)]
struct FSMeta {
    name: String,
    extension: String,
    created: String,
    modified: String,
    accessed: String,
    read_only: bool,
    // in KB
    size: u64,
}

impl FSMeta {
    fn from_entry(entry: DirEntry) -> Self {
        let meta = entry.metadata().unwrap();
        FSMeta {
            name: Self::name(&entry),
            extension: Self::extension(&entry),
            created: Self::created(&meta),
            accessed: Self::accessed(&meta),
            modified: Self::modified(&meta),
            read_only: Self::read_only(&meta),
            size: Self::size(&meta),
        }
    }

    fn name(entry: &DirEntry) -> String {
        entry.file_name().into_string().unwrap()
    }

    fn size(meta: &Metadata) -> u64 {
        // in KB
        meta.len() / 1024
    }

    fn created(meta: &Metadata) -> String {
        <SystemTime as Into<DateTime<Utc>>>::into(meta.created().unwrap()).to_string()
    }

    fn accessed(meta: &Metadata) -> String {
        <SystemTime as Into<DateTime<Utc>>>::into(meta.accessed().unwrap()).to_string()
    }

    fn modified(meta: &Metadata) -> String {
        <SystemTime as Into<DateTime<Utc>>>::into(meta.modified().unwrap()).to_string()
    }

    // returns empty string if the extension method returns None for any reason
    // otherwise returns the file extension
    fn extension(entry: &DirEntry) -> String {
        entry
            .path()
            .extension()
            .unwrap_or(&std::ffi::OsStr::new(""))
            .to_str()
            .unwrap()
            .to_string()
    }

    fn read_only(meta: &Metadata) -> bool {
        meta.permissions().readonly()
    }
}

#[get("/files-meta/{dir}")]
pub async fn files_meta(dir: web::Path<String>) -> ActixResult<impl Responder> {
    Ok(web::Json(inspect_dir(dir.as_str())))
}

// FIXME this function is broken
// do later
// use chronos now
// fn time_stamp_disabled(dura: Duration) {
//     let (mins, secs) = (dura / 60, dura % 60);
//     let (hours, mins) = (mins / 60, mins % 60);
//     let (days, hours) = (hours / 24, hours % 24);
//
//     // FIXME this would need to be done per year
//     // while accounting for February
//
//     let month_rem = ((31 * 7) + (30 * 4) + 28) % 12;
//     let month_rem_leap = ((31 * 7) + (30 * 4) + 29) % 12;
//     let month_len = 30;
//     eprintln!("> {} + {}", month_len, ((31 * 7) + (30 * 4) + 28) % 12);
//     eprintln!("> {} + {}", month_len, ((31 * 7) + (30 * 4) + 29) % 12);
//
//     let year_len = 365;
//     let year_len_leap = 366;
//
//     let (mut days_chunks, days_remains) = (days / 4, days % 4);
//
//     let mut days = 0;
//     let mut months = 0;
//     let mut year_cycle = 1;
//
//     while days_chunks > 0 {
//         if year_cycle == 4 {
//             months += year_len_leap / (month_len + month_rem_leap);
//             days += year_len_leap % (month_len + month_rem_leap);
//             year_cycle = 1;
//         } else {
//             months += year_len / (month_len + month_rem);
//             days += year_len % (month_len + month_rem);
//             year_cycle += 1;
//         }
//
//         days_chunks -= 1;
//     }
//
//     // (months, days) = (days / 30, days % 30);
//
//     (months, days) = if year_cycle == 4 {
//         (
//             months + (days_remains / (month_len + month_rem_leap)),
//             days + (days_remains % (month_len + month_rem_leap)),
//         )
//     } else {
//         (
//             months + (days_remains / (month_len + month_rem)),
//             days + (days_remains % (month_len + month_rem)),
//         )
//     };
//
//     let (years, months) = (months / 12, months % 12);
// }

pub(super) const FILES_MENU_ICONS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    ["bin-down.svg", "bin-up.svg", "bin-del.svg", "bin-add.svg"]
        .into_iter()
        .map(|icon| (icon, read_to_string(&format!("dist/app/icons/{}", icon))))
        .inspect(|(_, d)| {
            dbg!(&d);
        })
        .filter(|(_, d)| d.is_ok())
        .map(|(i, d)| (i.trim_end_matches(".svg").to_string(), d.unwrap()))
        .collect::<HashMap<String, String>>()
});
