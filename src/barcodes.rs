pub  mod barcode {
    #[derive(Debug, PartialEq)]
    pub enum BarcodeType {
        Code128,
        Pharmacode39,
        EAN13,
        Unknown
    }

    pub fn from_char(c : char) -> BarcodeType{
        match c {
            'F' => BarcodeType::EAN13,
            '#' => BarcodeType::Code128,
            _ => BarcodeType::Unknown
        }
    }
}