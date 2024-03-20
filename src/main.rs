use qrcodegen::{QrCode, QrCodeEcc};

fn main() {
    let qr = QrCode::encode_text("test text", QrCodeEcc::Medium).unwrap();
    let svg = to_svg_string(&qr, 10);

    println!("{svg}");
}

/* This is not our code and so not our property.
The code below is from nayuki (https://github.com/nayuki/QR-Code-generator) */

/*---- Utilities ----*/

// Returns a string of SVG code for an image depicting
// the given QR Code, with the given number of border modules.
// The string always uses Unix newlines (\n), regardless of the platform.
fn to_svg_string(qr: &QrCode, border: i32) -> String {
    assert!(border >= 0, "Border must be non-negative");
    let mut result = String::new();
    let dimension = qr
        .size()
        .checked_add(border.checked_mul(2).unwrap())
        .unwrap();
    result += &format!(
		"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n", dimension);
    result += "\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n";
    result += "\t<path d=\"";
    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    result += " ";
                }
                result += &format!("M{},{}h1v1h-1z", x + border, y + border);
            }
        }
    }
    result += "\" fill=\"#000000\"/>\n";
    result += "</svg>\n";
    result
}
