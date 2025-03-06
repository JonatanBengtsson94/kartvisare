use crate::model::wms::Wms;
use std::sync::{Arc, Mutex};

pub trait WmsRepository {
    fn get_all(&self) -> Vec<Wms>;
    fn get_by_id(&self, id: u32) -> Option<Wms>;
    fn add(&self, wms: Wms);
}

pub struct InMemoryWmsRepository {
    wms_list: Arc<Mutex<Vec<Wms>>>,
}

impl InMemoryWmsRepository {
    pub fn new() -> Self {
        InMemoryWmsRepository {
            wms_list: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl WmsRepository for InMemoryWmsRepository {
    fn get_all(&self) -> Vec<Wms> {
        let wms_list = self.wms_list.lock().unwrap();
        return wms_list.clone();
    }

    fn get_by_id(&self, id: u32) -> Option<Wms> {
        let wms_list = self.wms_list.lock().unwrap();
        for wms in wms_list.iter() {
            if wms.id == id {
                return Some(wms.clone());
            }
        }
        return None;
    }

    fn add(&self, wms: Wms) {
        let mut wms_list = self.wms_list.lock().unwrap();
        wms_list.push(wms);
    }
}
