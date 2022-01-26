use std::collections::HashMap;

use lazy_static::lazy_static;

pub const HEADER_LOC: usize = 0x0100;
pub const HEADER_SIZE: usize = 0x50;

const TITLE_LOC: usize = 0x34;
const DMG_TITLE_SIZE: usize = 0x10;
const CGB_TITLE_SIZE: usize = 0x0B;

const CGB_FLAG_LOC: usize = 0x43;
const SGB_FLAG_LOC: usize = 0x46;

const NEW_LIC_LOC: usize = 0x44;
const NEW_LIC_SIZE: usize = 0x02;
const OLD_LIC_LOC: usize = 0x4B;

const CART_TYPE_LOC: usize = 0x47;

const ROM_SIZE_LOC: usize = 0x48;
const RAM_SIZE_LOC: usize = 0x49;

const DST_LOC: usize = 0x4A;

const VERSION_LOC: usize = 0x4C;

const CHECKSUM_LOC: usize = 0x4D;
const CHECKSUM_START: usize = 0x34;
const CHECKSUM_END: usize = 0x4D;

pub struct Header {
    pub title: String,
    pub licensee: &'static str,
    pub cgb_flag: u8,
    pub sgb_flag: u8,
    pub cart_type: &'static str,
    pub rom_size: u32,
    pub n_banks: u32,
    pub ram_size: u32,
    pub dst: &'static str,
    pub version: u8,
    pub checksum: bool,
}

impl Header {
    pub fn new(buf: &[u8]) -> Self {
        assert_eq!(buf.len(), HEADER_SIZE);

        let title_end = TITLE_LOC + title_size(buf[CGB_FLAG_LOC]);
        let slice = &buf[TITLE_LOC..title_end];
        let title = String::from_utf8_lossy(slice).to_string();

        Self {
            title,
            licensee: licensee(
                buf[OLD_LIC_LOC],
                &buf[NEW_LIC_LOC..NEW_LIC_LOC + NEW_LIC_SIZE],
            ),
            cart_type: cart_type(buf[CART_TYPE_LOC]),
            cgb_flag: buf[CGB_FLAG_LOC],
            sgb_flag: buf[SGB_FLAG_LOC],
            rom_size: 32 << buf[ROM_SIZE_LOC],
            n_banks: 2 << buf[ROM_SIZE_LOC],
            ram_size: ram_size(buf[RAM_SIZE_LOC]),
            dst: destination(buf[DST_LOC]),
            version: buf[VERSION_LOC],
            checksum: checksum(buf[CHECKSUM_LOC], &buf[CHECKSUM_START..CHECKSUM_END]),
        }
    }

    pub fn print(&self, width: usize) {
        println!("{:<width$} {}", "Title:", self.title, width = width);
        println!("{:<width$} {}", "Licensee:", self.licensee, width = width);
        println!("{:<width$} {}", "Type:", self.cart_type, width = width);

        let (rom_size, unit) = match self.rom_size > 512 {
            true => (format!("{}", self.rom_size / 1024), "MBs"),
            false => (format!("{}", self.rom_size), "KBs"),
        };
        println!(
            "{:<width$} {} {}",
            "ROM Size:",
            rom_size,
            unit,
            width = width
        );
        println!("{:<width$} {}", "ROM Banks:", self.n_banks, width = width);

        let (ram_size, unit) = match self.ram_size {
            0 => ("No RAM".to_string(), ""),
            _ => (format!("{}", self.ram_size), "KBs"),
        };
        println!(
            "{:<width$} {} {}",
            "RAM Size:",
            ram_size,
            unit,
            width = width
        );
        if self.ram_size > 0 {
            println!(
                "{:<width$} {}",
                "RAM Banks:",
                self.ram_size / 8,
                width = width
            );
        }
        println!("{:<width$} {}", "Region:", self.dst, width = width);
        println!("{:<width$} {}", "Version:", self.version, width = width);

        let result = match self.checksum {
            true => "Ok",
            false => "Failed",
        };
        println!("{:<width$} {}", "Checksum:", result, width = width);
    }
}

fn licensee(old: u8, new: &[u8]) -> &'static str {
    assert_eq!(new.len(), NEW_LIC_SIZE);

    let hi = new[0] & 0x0F;
    let lo = new[1] & 0x0F;
    let new_lic_code = hi << 4 | lo;
    match old == 0x33 {
        true => *NEW_LICENSEES.get(&new_lic_code).unwrap_or(&"None"),
        false => *OLD_LICENSEES.get(&old).unwrap_or(&"None"),
    }
}

fn cart_type(code: u8) -> &'static str {
    *CART_TYPES.get(&code).unwrap_or(&"ROM ONLY")
}

const fn title_size(code: u8) -> usize {
    // 0b1000_0000 0b1100_0000
    match code & 0xF0 == 0x80 || code & 0xF0 == 0xC0 {
        true => CGB_TITLE_SIZE,
        false => DMG_TITLE_SIZE,
    }
}

const fn ram_size(code: u8) -> u32 {
    match code {
        0x02 => 8,
        0x03 => 32,
        0x04 => 128,
        0x05 => 64,
        _ => 0,
    }
}

const fn destination(code: u8) -> &'static str {
    match code {
        0x00 => "Japanese",
        _ => "Non-Japanese",
    }
}

fn checksum(sum: u8, bytes: &[u8]) -> bool {
    assert_eq!(bytes.len(), 0x4D - 0x34);

    let x = bytes
        .iter()
        .fold(0u8, |acc, &v| acc.wrapping_sub(v).wrapping_sub(1));

    x == sum
}

lazy_static! {
    static ref CART_TYPES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();

        m.insert(0x01, "MBC1");
        m.insert(0x02, "MBC1+RAM");
        m.insert(0x03, "MBC1+RAM+BATTERY");
        m.insert(0x05, "MBC2");
        m.insert(0x06, "MBC2+BATTERY");
        m.insert(0x08, "ROM+RAM");
        m.insert(0x09, "ROM+RAM+BATTERY");
        m.insert(0x0B, "MMM01");
        m.insert(0x0C, "MMM01+RAM");
        m.insert(0x0D, "MMM01+RAM+BATTERY");
        m.insert(0x0F, "MBC3+TIMER+BATTERY");
        m.insert(0x10, "MBC3+TIMER+RAM+BATTERY");
        m.insert(0x11, "MBC3");
        m.insert(0x12, "MBC3+RAM");
        m.insert(0x13, "MBC3+RAM+BATTERY");
        m.insert(0x19, "MBC5");
        m.insert(0x1A, "MBC5+RAM");
        m.insert(0x1B, "MBC5+RAM+BATTERY");
        m.insert(0x1C, "MBC5+RUMBLE");
        m.insert(0x1D, "MBC5+RUMBLE+RAM");
        m.insert(0x1E, "MBC5+RUMBLE+RAM+BATTERY");
        m.insert(0x20, "MBC6");
        m.insert(0x22, "MBC7+SENSOR+RUMBLE+RAM+BATTERY");
        m.insert(0xFC, "POCKET CAMERA");
        m.insert(0xFD, "BANDAI TAMA5");
        m.insert(0xFE, "HuC3");
        m.insert(0xFF, "HuC1+RAM+BATTERY");

        m
    };
    static ref NEW_LICENSEES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();

        m.insert(0x01, "Nintendo");
        m.insert(0x08, "Capcom");
        m.insert(0x13, "Electronic Arts");
        m.insert(0x18, "Hudson Soft");
        m.insert(0x19, "B-AI");
        m.insert(0x20, "KSS");
        m.insert(0x22, "Pow");
        m.insert(0x24, "PCM Complete");
        m.insert(0x25, "San-X");
        m.insert(0x28, "Kemco");
        m.insert(0x29, "SETA");
        m.insert(0x30, "Viacom");
        m.insert(0x31, "Nintendo");
        m.insert(0x32, "Bandai");
        m.insert(0x33, "Ocean/Acclaim");
        m.insert(0x34, "Konami");
        m.insert(0x35, "Hector");
        m.insert(0x37, "Taito");
        m.insert(0x38, "Hudson Soft");
        m.insert(0x39, "Banpresto");
        m.insert(0x41, "Ubi Soft");
        m.insert(0x42, "Atlus");
        m.insert(0x44, "Malibu");
        m.insert(0x46, "Angel");
        m.insert(0x47, "Bullet-Proof");
        m.insert(0x49, "Irem");
        m.insert(0x50, "Absolute");
        m.insert(0x51, "Acclaim");
        m.insert(0x52, "Activision");
        m.insert(0x53, "American Sammy");
        m.insert(0x54, "Konami");
        m.insert(0x55, "Hi Tech Entertainment");
        m.insert(0x56, "LJN");
        m.insert(0x57, "Matchbox");
        m.insert(0x58, "Mattel");
        m.insert(0x59, "Milton Bradley");
        m.insert(0x60, "Titus");
        m.insert(0x61, "Virgin");
        m.insert(0x64, "LucasArts");
        m.insert(0x67, "Ocean");
        m.insert(0x69, "Electronic Arts");
        m.insert(0x70, "Infogrames");
        m.insert(0x71, "Interplay");
        m.insert(0x72, "Broderbund");
        m.insert(0x73, "Sculptured Software");
        m.insert(0x75, "The Sales Curve");
        m.insert(0x78, "THQ");
        m.insert(0x79, "Accolade");
        m.insert(0x80, "Misawa");
        m.insert(0x83, "LOZC");
        m.insert(0x86, "Tokuma Shoten Intermedia");
        m.insert(0x87, "Tsukuda Original");
        m.insert(0x91, "Chunsoft");
        m.insert(0x92, "Video System");
        m.insert(0x93, "Ocean/Acclaim");
        m.insert(0x95, "Varie");
        m.insert(0x96, "Yonezawa/s’pal");
        m.insert(0x97, "Kaneko");
        m.insert(0x99, "Pack in soft");
        m.insert(0xA4, "Konami (Yu-Gi-Oh!)");

        m
    };
    static ref OLD_LICENSEES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();

        m.insert(0x01, "Nintendo");
        m.insert(0x08, "Capcom");
        m.insert(0x09, "Hot-B");
        m.insert(0x0A, "Jaleco");
        m.insert(0x0B, "Coconuts Japan");
        m.insert(0x0B, "Elite Systems");
        m.insert(0x13, "Electronic Arts");
        m.insert(0x18, "Hudson Soft");
        m.insert(0x19, "ITC entertainment");
        m.insert(0x1A, "Yanoman");
        m.insert(0x1D, "Clary");
        m.insert(0x1F, "Virgin");
        m.insert(0x24, "PCM Complete");
        m.insert(0x25, "San-X");
        m.insert(0x28, "Kotobuki Systems");
        m.insert(0x29, "SETA");
        m.insert(0x30, "Infogrames");
        m.insert(0x31, "Nintendo");
        m.insert(0x32, "Bandai");
        m.insert(0x34, "Konami");
        m.insert(0x35, "Hector");
        m.insert(0x38, "Capcom");
        m.insert(0x39, "Banpresto");
        m.insert(0x3C, "Entertainment Int");
        m.insert(0x3E, "Gremlin");
        m.insert(0x41, "Ubi Soft");
        m.insert(0x42, "Atlus");
        m.insert(0x44, "Malibu");
        m.insert(0x46, "Angel");
        m.insert(0x47, "Spectrum Holoby");
        m.insert(0x49, "irem");
        m.insert(0x4A, "Virgin");
        m.insert(0x4D, "Malibu");
        m.insert(0x4F, "U.S. Gold");
        m.insert(0x50, "Absolute");
        m.insert(0x51, "Acclaim");
        m.insert(0x52, "Activision");
        m.insert(0x53, "American Sammy");
        m.insert(0x54, "GameTek");
        m.insert(0x55, "Park Place");
        m.insert(0x56, "LJN");
        m.insert(0x57, "Matchbox");
        m.insert(0x59, "Milton Bradley");
        m.insert(0x5A, "Mindscape");
        m.insert(0x5B, "Romstar");
        m.insert(0x5C, "Naxat Soft");
        m.insert(0x5D, "Tradewest");
        m.insert(0x60, "Titus");
        m.insert(0x61, "Virgin");
        m.insert(0x67, "Ocean");
        m.insert(0x69, "Electronic Arts");
        m.insert(0x6E, "Elite Systems");
        m.insert(0x6F, "Electro Brain");
        m.insert(0x70, "Infogrames");
        m.insert(0x71, "Interplay");
        m.insert(0x72, "Broderbund");
        m.insert(0x73, "Sculptured Software");
        m.insert(0x75, "The Sales Curve");
        m.insert(0x78, "THQ");
        m.insert(0x79, "Accolade");
        m.insert(0x7A, "Triffix");
        m.insert(0x7C, "Microprose");
        m.insert(0x7F, "Kemco");
        m.insert(0x80, "Misawa");
        m.insert(0x83, "LOZC");
        m.insert(0x86, "Tokuma Shoten Intermedia");
        m.insert(0x8B, "Bullet-Proof");
        m.insert(0x8C, "Vic Tokai");
        m.insert(0x8E, "Ape");
        m.insert(0x8F, "I'Max");
        m.insert(0x91, "Chunsoft");
        m.insert(0x92, "Video system");
        m.insert(0x93, "tsuburava");
        m.insert(0x95, "Varie");
        m.insert(0x96, "Yonezawa/s’pal");
        m.insert(0x97, "Kaneko");
        m.insert(0x99, "Arc");
        m.insert(0x9A, "Nihon Bussan");
        m.insert(0x9B, "Tecmo");
        m.insert(0x9C, "Imagineer");
        m.insert(0x9D, "Banpresto");
        m.insert(0x9F, "Nova");
        m.insert(0xA1, "Hori Electric");
        m.insert(0xA2, "Bandai");
        m.insert(0xA4, "Konami (Yu-Gi-Oh!)");
        m.insert(0xA6, "Kawada");
        m.insert(0xA7, "Takara");
        m.insert(0xA9, "Technōs Japan");
        m.insert(0xAA, "Broderbund");
        m.insert(0xAC, "Toei Animation");
        m.insert(0xAD, "Toho");
        m.insert(0xAF, "Namco");
        m.insert(0xB0, "Acclaim");
        m.insert(0xB1, "Ascii/Nexoft");
        m.insert(0xB2, "Bandai");
        m.insert(0xB4, "Enix");
        m.insert(0xB6, "HAL");
        m.insert(0xB7, "SNK");
        m.insert(0xB9, "Pony Canyon");
        m.insert(0xBA, "Culture Brain");
        m.insert(0xBB, "Sunsoft");
        m.insert(0xBD, "Sony Imagesoft");
        m.insert(0xBF, "American Sammy");
        m.insert(0xC0, "Taito");
        m.insert(0xC2, "Kemco");
        m.insert(0xC3, "SquareSoft");
        m.insert(0xC4, "Tokuma Shoten Intermedia");
        m.insert(0xC5, "Data East");
        m.insert(0xC6, "Tonkin House");
        m.insert(0xC8, "Koei");
        m.insert(0xC9, "UFL");
        m.insert(0xCA, "Ultra Games");
        m.insert(0xCB, "VAP");
        m.insert(0xCC, "Use Corporation");
        m.insert(0xCD, "Meldac");
        m.insert(0xCE, "Pony Canyon");
        m.insert(0xCF, "Angel");
        m.insert(0xD0, "Taito");
        m.insert(0xD1, "SOFEL");
        m.insert(0xD2, "Quest");
        m.insert(0xD3, "Sigma Enterprises");
        m.insert(0xD4, "ASK Kodansha       ");
        m.insert(0xD6, "Naxat Soft");
        m.insert(0xD7, "Copya Systems");
        m.insert(0xD9, "Banpresto");
        m.insert(0xDA, "Tomy");
        m.insert(0xDB, "LJN");
        m.insert(0xDD, "NCS");
        m.insert(0xDE, "Human Entertainment");
        m.insert(0xDF, "Altron");
        m.insert(0xE0, "Jaleco");
        m.insert(0xE1, "Towachiki");
        m.insert(0xE2, "Uutaka");
        m.insert(0xE3, "Varie");
        m.insert(0xE5, "Epoch");
        m.insert(0xE7, "Athena");
        m.insert(0xE8, "Asmik");
        m.insert(0xE9, "Natsume");
        m.insert(0xEA, "King Records");
        m.insert(0xEB, "Atlus");
        m.insert(0xEC, "Epic/Sony Records");
        m.insert(0xEE, "IGS");
        m.insert(0xF0, "A Wave");
        m.insert(0xF3, "Extreme Entertainment");
        m.insert(0xFF, "LJN");

        m
    };
}
