#[derive(Clone, Default)]
pub struct ModemSpecs {
    pub _manufacturer: &'static str,
    pub _model: &'static str,
    pub supported_umts_bands: &'static [usize],
    pub supported_lte_bands: &'static [usize],
}

pub static FIBOCOM_L850GL: ModemSpecs = ModemSpecs {
    _manufacturer: "Fibocom",
    _model: "L850",
    supported_umts_bands: &[1,2,4,5,8],
    supported_lte_bands: &[1,2,3,4,5,7,8,12,13,17,18,19,20,26,28,29,30,41,66]
};

#[derive(Clone, Default)]
pub struct Modem {
    pub(crate) spec: Option<&'static ModemSpecs>,
}

impl Modem {
    pub fn new(model: &str) -> Result<&'static ModemSpecs, &'static str> {
        match model {
            "L850" => Ok(&FIBOCOM_L850GL),
            _ => Err("Modem is not supported")
        }
    }
}