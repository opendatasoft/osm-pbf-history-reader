use chrono::{DateTime, TimeZone, Utc};
use std::fmt;
use std::fmt::Formatter;

/// Represents infos about one version of an OSM element
pub struct Info {
    pub version: i32,
    pub changeset: i64,
    pub uid: i32,
    pub timestamp: i64,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "\t{} changeset :{} - uid: {} - time: {}",
            self.version,
            self.changeset,
            self.uid,
            Utc.timestamp_millis_opt(self.timestamp).unwrap(),
        )?;
        Ok(())
    }
}

/// Represents infos of every versions of an OSM element
pub struct GatheredInfos {
    pub timestamps: Vec<DateTime<Utc>>,
    pub uids: Vec<i32>,
    pub changesets: Vec<i64>,
}

impl GatheredInfos {
    pub fn new() -> GatheredInfos {
        GatheredInfos {
            timestamps: vec![],
            uids: vec![],
            changesets: vec![],
        }
    }

    pub fn add_info(&mut self, info: Info) {
        self.timestamps
            .push(Utc.timestamp_millis_opt(info.timestamp).unwrap());
        self.changesets.push(info.changeset);
        self.uids.push(info.uid);
    }
}

impl fmt::Display for GatheredInfos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f)?;
        write!(f, "\t  Timestamps: ")?;
        for i in &self.timestamps {
            write!(f, "{} - ", *i)?;
        }

        write!(f, "\n\t  Users: ")?;
        write!(f, "\n\t  Changesets: ")?;
        for i in &self.changesets {
            write!(f, "{} - ", i)?;
        }
        write!(f, "\n\t  Visible: ")?;
        Ok(())
    }
}
