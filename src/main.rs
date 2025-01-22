use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;
use time::{Date, Duration};
use sha1::{Sha1, Digest};
use hmac::{Hmac, Mac};
use zip::ZipWriter;

fn main() -> io::Result<()> {
    let templates: HashMap<char, &str> = [
        ('U', "templateU.bin"),
        ('E', "templateE.bin"),
        ('J', "templateJ.bin"),
        ('K', "templateK.bin"),
    ]
    .iter()
    .cloned()
    .collect();

    let bundlebase = Path::new("bundle");

    hax(&templates, bundlebase)
}

fn hax(templates: &HashMap<char, &str>, bundlebase: &Path) -> io::Result<()> {
    let dt = Date::from_ordinal_date(2006, 324).unwrap() - Duration::days(1);
    let delta = (dt - Date::from_ordinal_date(2000, 1).unwrap()).whole_seconds();
    let timestamp = delta as u32;

    println!("Please Enter Wii Mac Address: (ex. 1111111111111111)");
    let mut mac = String::new();
    io::stdin().read_line(&mut mac)?;
    let mac = mac.trim().to_lowercase();

    println!("Please Enter Wii Region: (ex. U = US E = EUR J = JAP K = KOR)");
    let template_key = {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.trim().to_uppercase().chars().next()
    };

    if let Some(template) = templates.get(&template_key.unwrap_or_default()) {
        println!("Bundle with Hackmii? 1 = Yes 0 = No");
        let mut bundle = String::new();
        io::stdin().read_line(&mut bundle)?;
        let bundle = bundle.trim() == "1";

        if mac.as_bytes() == b"\x00\x17\xab\x99\x99\x99" {
            println!("If you're using Dolphin, try File->Open instead ;-).");
        }

        let mut valid_mac = false;
        let oui_list_file = File::open("oui_list.txt")?;
        for line in io::BufReader::new(oui_list_file).lines() {
            if mac.starts_with(&line?.trim().to_lowercase()) {
                valid_mac = true;
                break;
            }
        }

        if !valid_mac {
            println!("Invalid MAC Quitting...");
            return Ok(());
        }

        let bytes: Vec<u8> = (0..6)
            .map(|i| u8::from_str_radix(&mac[i * 2..i * 2 + 2], 16).unwrap())
            .collect();

        let mut hasher = Sha1::new();
        hasher.update(&bytes);
        hasher.update(&[0x75, 0x79, 0x79]);
        let key = hasher.finalize();

        let mut blob: Vec<u8> = fs::read(template)?;

        blob[0x08..0x10].copy_from_slice(&key[..8]);
        blob[0xb0..0xc4].fill(0);
        blob[0x7c..0x80].copy_from_slice(&timestamp.to_be_bytes());

        let mut hmac = Hmac::<Sha1>::new_from_slice(&key[8..]).unwrap();
        hmac.update(&blob);
        blob[0xb0..0xc4].copy_from_slice(&hmac.finalize().into_bytes());

        let path = format!(
            "private/wii/title/HAEA/{:02X}{:02X}/{:02X}{:02X}/{}/{:02}/{:02}/HABA_#1/txt/{:08X}.000",
            key[0], key[1], key[2], key[3],
            dt.year(), dt.month() as u8, dt.day() as u8,
            timestamp
        );

        let mut zip = ZipWriter::new(File::create("BirthdayLetter.zip")?);
        zip.start_file(path, Default::default())?;
        zip.write_all(&blob)?;

        if bundle {
            for entry in fs::read_dir(bundlebase)? {
                let entry = entry?;
                if !entry.path().is_dir() && !entry.file_name().to_string_lossy().starts_with('.') {
                    zip.start_file(entry.file_name().to_string_lossy(), Default::default())?;
                    zip.write_all(&fs::read(entry.path())?)?;
                }
            }
        }

        println!("Complete, Birthday Letter Generated as BirthdayLetter.zip.");
        println!("Please Extract to the root of your SD Card");
        println!("Don't Forget to set the time to November 19, 2006.");
    } else {
        println!("Invalid template key.");
    }

    Ok(())
}
