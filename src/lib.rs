// Include the generated constant file
include!("hepevt_size.rs");
//
//#[repr(C)]
#[allow(non_upper_case_globals)]
#[repr(C)]
pub struct HEPEVTCommonBlock<const N: usize> {
    // Scalars
    pub nevhep_: i32,
    pub nhep_: i32,

    // 1D arrays
    pub isthep_: [i32; N],
    pub idhep_: [i32; N],

    // 2D arrays (FORTRAN column-major: [rows][cols])
    pub jmohep_: [[i32; 2]; N],
    pub jdahep_: [[i32; 2]; N],

    // Double precision arrays
    pub phep_: [[f64; 5]; N],
    pub vhep_: [[f64; 4]; N],
}

#[link(name = "common_block")]  
unsafe extern "C" {
    #[link_name = "hepevt_"]
    pub static mut hepevt_: HEPEVTCommonBlock<NMXHEP>;
}

#[link(name = "common_block")]  
unsafe extern "C" {
    #[allow(dead_code)]
    fn hepevt_dummy_function_() -> i32;
}


#[derive(Debug, Clone)]
pub struct HEPEVT<const N: usize> {
    pub nevhep: i32,
    pub nhep: i32,
    pub isthep: [i32; N],
    pub idhep: [i32; N],
    pub jmohep: [[i32; 2]; N],
    pub jdahep: [[i32; 2]; N],
    pub phep: [[f64; 5]; N],
    pub vhep: [[f64; 4]; N],
}


impl<const N:usize> HEPEVT<N> {
    pub fn new() -> Self {
        HEPEVT {
            nevhep: 0,
            nhep: 0,
            idhep: [0; N],
            isthep: [0; N],
            jmohep: [[0; 2]; N],
            jdahep: [[0; 2]; N],
            phep: [[0.0; 5]; N],
            vhep: [[0.0; 4]; N],
        }
    }
}



impl<const N:usize> HEPEVT<N> {
    pub fn copy_from_custom_common_block(hepevt : *const HEPEVTCommonBlock<N> ) -> Self {
        let hepevt = unsafe { &*hepevt };
        HEPEVT {
            nevhep: hepevt.nevhep_,
            nhep: hepevt.nhep_,
            isthep: hepevt.isthep_,
            idhep: hepevt.idhep_,
            jmohep: hepevt.jmohep_,
            jdahep: hepevt.jdahep_,
            phep: hepevt.phep_,
            vhep: hepevt.vhep_,
        }
    }

    pub fn copy_to_custom_common_block(&self, hepevt : *mut HEPEVTCommonBlock<N>) {
        let hepevt = unsafe { &mut *hepevt };
        hepevt.nevhep_ = self.nevhep;
        hepevt.nhep_ = self.nhep;
        for i in 0..NMXHEP {
            hepevt.isthep_[i] = self.isthep[i];
            hepevt.idhep_[i] = self.idhep[i];
            hepevt.jmohep_[i][0] = self.jmohep[i][0];
            hepevt.jmohep_[i][1] = self.jmohep[i][1];
            hepevt.jdahep_[i][0] = self.jdahep[i][0];
            hepevt.jdahep_[i][1] = self.jdahep[i][1];
            for j in 0..5 {
                hepevt.phep_[i][j] = self.phep[i][j];
            }
            for j in 0..4 {
                hepevt.vhep_[i][j] = self.vhep[i][j];
            }
        }
    }
}


impl HEPEVT<NMXHEP> {
    pub fn copy_from_common_block() -> Self {
        return HEPEVT::copy_from_custom_common_block(  &raw const hepevt_ );
    }

    pub fn copy_to_common_block(&self)  {
        self.copy_to_custom_common_block( &raw mut hepevt_);
    }
}

#[cfg(test)]
mod tests {

    use serial_test::serial;
    use super::*;

    #[test]
    #[serial]
    fn test_hepevt_new() {
        let evt = HEPEVT::<NMXHEP>::new();
        assert_eq!(evt.nevhep, 0);
        assert_eq!(evt.nhep, 0);
        assert_eq!(evt.idhep[0], 0);
    }

    #[test]
    #[serial]
    fn test_store_and_load() {
        // Initialize HEPEVT and modify some values
        let mut evt = HEPEVT::new();
        evt.nevhep = 42;
        evt.nhep = 2;
        evt.idhep[0] = 11; // e.g., electron
        evt.phep[0][0] = 100.0; // px for first particle

        // Store to common block (unsafe interaction)
        evt.copy_to_common_block();

        // Load from common block
        let evt_loaded = HEPEVT::copy_from_common_block();

        assert_eq!(evt_loaded.nevhep, 42);
        assert_eq!(evt_loaded.nhep, 2);
        assert_eq!(evt_loaded.idhep[0], 11);
        assert_eq!(evt_loaded.phep[0][0], 100.0);
    }

    #[test]
    #[serial]
    fn test_read() {
        // Initialize HEPEVT and modify some values
        unsafe {
            assert_eq!(hepevt_dummy_function_(), NMXHEP as i32);
        }
        // Load from common block
        let evt_loaded = HEPEVT::copy_from_custom_common_block(&raw const hepevt_);

        assert_eq!(evt_loaded.nevhep, 43);
        assert_eq!(evt_loaded.jmohep[0][0], 11);
        assert_eq!(evt_loaded.jmohep[0][1], 21);

        assert_eq!(evt_loaded.jmohep[2][0], 13);
        assert_eq!(evt_loaded.jmohep[2][1], 23);
    }
}


