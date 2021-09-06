use crate::qucc::{Info, QuccBMS};
use std::error::Error;
use std::ops::Sub;
use std::time::{Duration, SystemTime};

const REFRESH_RATE: Duration = Duration::from_secs(5);

pub struct BMS {
    bms: QuccBMS,

    info_last_check_time: SystemTime,
    info: Info,

    cell_v: Vec<u16>,
    cell_v_last_check_time: SystemTime,
}

impl BMS {
    pub fn new(bms: QuccBMS) -> BMS {
        BMS {
            bms,
            info: Info::default(),
            info_last_check_time: SystemTime::now().sub(Duration::from_secs(60)),
            cell_v: Vec::new(),
            cell_v_last_check_time: SystemTime::now().sub(Duration::from_secs(60)),
        }
    }

    pub fn get_bms(&self) -> &QuccBMS {
        &self.bms
    }

    fn ready_to_refresh_info(&mut self) -> bool {
        let now = SystemTime::now();
        if now
            .duration_since(self.info_last_check_time)
            .unwrap()
            .ge(&REFRESH_RATE)
        {
            self.info_last_check_time = now;
            true
        } else {
            false
        }
    }

    fn ready_to_refresh_cell_v(&mut self) -> bool {
        let now = SystemTime::now();
        if now
            .duration_since(self.cell_v_last_check_time)
            .unwrap()
            .ge(&REFRESH_RATE)
        {
            self.cell_v_last_check_time = now;
            true
        } else {
            false
        }
    }

    pub fn get_cell_v(&mut self) -> Result<Vec<u16>, Box<dyn Error>> {
        if self.ready_to_refresh_cell_v() {
            let cell_v = self.bms.get_cell_v()?;
            self.cell_v = cell_v;
        }
        Ok(self.cell_v.clone())
    }

    pub fn get_info(&mut self) -> Result<&Info, Box<dyn Error>> {
        if self.ready_to_refresh_info() {
            self.info = self.bms.get_info()?;
            self.bms.set_cell_count(self.info.cell_count);
        }
        Ok(&self.info)
    }
}
