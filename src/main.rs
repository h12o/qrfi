use clap::{Parser, ValueEnum};
use qrcode::render::unicode;
use qrcode::QrCode;
use std::io::{self, Read, Write, Cursor, IsTerminal};
use image::{Luma, ImageBuffer, ImageFormat};

use qrfi::{Wifi, Ssid, Password, AuthType};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Default)]
enum Format {
    #[default]
    Ascii,
    Png,
    Svg,
}

#[derive(Parser, Debug)]
#[command(
    name = "qrfi",
    version,
    about = "A CLI Wi-Fi QR Code Generator",
    after_help = "\x1b[1;4mExamples:\x1b[0m\n  qrfi SSID -p PASSWORD\n  qrfi SSID -p PASSWORD -f png > qr.png\n  echo SSID | qrfi -p PASSWORD\n\nQR Code is a registered trademark of DENSO WAVE INCORPORATED in Japan and in other countries."
)]
struct Args {
    #[arg(help = "SSID of the Wi-Fi network (or via stdin)")]
    ssid: Option<String>,
    #[arg(short = 't', long, value_enum, default_value_t = AuthType::Wpa, help = "Wi-Fi Authentication type")]
    authentication_type: AuthType,
    #[arg(short = 'p', long, help = "Wi-Fi password (ignored if authentication-type is 'nopass')")]
    password: Option<String>,
    #[arg(short = 'H', long, default_value_t = false, help = "Option to specify when SSID is hidden")]
    hidden: bool,
    #[arg(short = 'f', long, value_enum, default_value_t = Format::Ascii, help = "Output format")]
    format: Format,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = Args::parse();
    if args.ssid.is_none() && !io::stdin().is_terminal() {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        args.ssid = Some(buffer.trim_end_matches(['\n', '\r']).to_string());
    }
    if args.authentication_type == AuthType::Nopass {
        args.password = None;
    }
    let wifi = Wifi {
        ssid: Ssid(args.ssid.unwrap_or_default()),
        password: Password {
            value: args.password,
            auth_type: args.authentication_type,
        },
        hidden: args.hidden,
    };
    wifi.validate()?;
    let mecard = wifi.to_mecard();
    let code = QrCode::new(&mecard)?;
    match args.format {
        Format::Ascii => {
            let image = code.render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Dark)
                .light_color(unicode::Dense1x2::Light)
                .build();
            println!("{}", image);
        }
        Format::Png => {
            let width = code.width() as u32;
            let quiet_zone = 4;
            let total_width = width + (quiet_zone * 2);
            let scale = 10; 
            let final_dim = total_width * scale;
            let mut img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(final_dim, final_dim);
            for pixel in img.pixels_mut() {
                *pixel = Luma([255]);
            }
            for (y, row) in code.to_colors().chunks(width as usize).enumerate() {
                for (x, color) in row.iter().enumerate() {
                    if color == &qrcode::types::Color::Dark {
                        let px = (x as u32 + quiet_zone) * scale;
                        let py = (y as u32 + quiet_zone) * scale;
                        for dx in 0..scale {
                            for dy in 0..scale {
                                img.put_pixel(px + dx, py + dy, Luma([0]));
                            }
                        }
                    }
                }
            }
            let mut buf = Cursor::new(Vec::new());
            img.write_to(&mut buf, ImageFormat::Png)?;
            io::stdout().write_all(buf.get_ref())?;
        }
        Format::Svg => {
            let svg_image = code.render()
                .min_dimensions(200, 200)
                .dark_color(qrcode::render::svg::Color("#000000"))
                .light_color(qrcode::render::svg::Color("#ffffff"))
                .build();
            println!("{}", svg_image);
        }
    }
    Ok(())
}
