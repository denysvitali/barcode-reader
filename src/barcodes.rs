
pub mod barcode {
    #[derive(Debug, PartialEq)]
    pub enum BarcodeType {
        Code128,
        Pharmacode39,
        EAN8,
        EAN13,
        QR,
        Unknown
    }

    pub fn from_char(c : char) -> BarcodeType{
        match c {
            'F' => BarcodeType::EAN13,
            '#' => BarcodeType::Code128,
            _ => BarcodeType::Unknown
        }
    }

    pub fn detect_type(s : &str) -> BarcodeType{
        if s.starts_with("FF") {
            return BarcodeType::EAN8;
        }

        if s.starts_with("QR") {
            return BarcodeType::QR;
        }

        return from_char(s.chars().nth(0).unwrap());
    }
}