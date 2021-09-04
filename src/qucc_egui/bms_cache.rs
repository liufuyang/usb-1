use crate::qucc::QuccBMS;
use std::error::Error;
use std::ops::Sub;
use std::time::{Duration, SystemTime};

const REFRESH_RATE: Duration = Duration::from_secs(5);

pub struct BMS {
    bms: QuccBMS,
    last_check_time: SystemTime,
    cell_v: Vec<u16>,
}

impl BMS {
    pub fn new(bms: QuccBMS) -> BMS {
        BMS {
            bms,
            last_check_time: SystemTime::now().sub(Duration::from_secs(60)),
            cell_v: Vec::new(),
        }
    }

    pub fn get_bms(&self) -> &QuccBMS {
        &self.bms
    }

    fn ready_to_refresh(&mut self) -> bool {
        let now = SystemTime::now();
        if now
            .duration_since(self.last_check_time)
            .unwrap()
            .ge(&REFRESH_RATE)
        {
            self.last_check_time = now;
            true
        } else {
            false
        }
    }

    pub fn get_cell_v(&mut self) -> Result<Vec<u16>, Box<dyn Error>> {
        if self.ready_to_refresh() {
            let cell_v = self.bms.get_cell_v()?;
            self.cell_v = cell_v;
        }
        Ok(self.cell_v.clone())
    }
}
