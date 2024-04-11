use crate::tools::s_to_u32;
use image::Rgb;
use std::{collections::HashMap, error::Error, fmt::Display, fs, io::Read, path::Path};

pub mod paint;
#[derive(Debug, Clone)]
pub struct Printer {
    color_in: Rgb<u8>,
    color_out: Rgb<u8>,
    gap: u32,
    size: u32,
    char_data: CharData,
}
impl Printer {
    pub fn defult() -> Self {
        Self {
            color_in: Rgb([0, 0, 0]),
            color_out: Rgb([255, 255, 255]),
            gap: 15,
            char_data: CharData::defult(),
            size: 8,
        }
    }
    pub fn load<T: AsRef<Path> + Copy>(&mut self, path: T) -> Result<(), Box<dyn Error>> {
        self.char_data = CharData::new(path)?;
        Ok(())
    }
    pub fn set_color_in(&mut self, color_in: Rgb<u8>) {
        self.color_in = color_in;
    }
    pub fn set_color_out(&mut self, color_out: Rgb<u8>) {
        self.color_out = color_out;
    }
    pub fn set_size(&mut self, size: u32) {
        self.size = size;
    }
    pub fn set_gap(&mut self, gap: u32) {
        self.gap = gap;
    }
    pub fn get_now_char_data(&self) ->(Option<String>,usize){
        (self.char_data.name.clone(),self.char_data.get_len())
    }
}
#[derive(Debug, Clone)]
struct CharData {
    name: Option<String>,
    len: usize,
    height: u32,
    data: HashMap<String, SingleChar>,
}
impl CharData {
    pub fn new<T: AsRef<Path> + Copy>(path: T) -> Result<Self, Box<dyn Error>> {
        let mut file = fs::File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut len = 0;
        let mut char_map = HashMap::new();
        let mut block = data.split_ascii_whitespace();
        let mut height = 0;
        loop {
            let key = match block.next() {
                Some(t) => t.to_string(),
                None => break,
            };
            let w = match block.next() {
                Some(t) => s_to_u32(t),
                None => return Err("CharDataFmtERROR".into()),
            };
            let h = match block.next() {
                Some(t) => s_to_u32(t),
                None => return Err("CharDataFmtERROR".into()),
            };
            let char_data = match block.next() {
                Some(t) => {
                    let mut s = t.to_string();
                    let mut v = vec![];
                    while let Some(i) = s.pop() {
                        if i == '1' {
                            v.push(true);
                        } else {
                            v.push(false);
                        }
                    }
                    //v.reverse();
                    v
                }
                None => return Err("CharDataFmtERROR".into()),
            };
            len += 1;
            if h > height {
                height = h;
            }
            char_map.insert(
                key,
                SingleChar {
                    height: h,
                    width: w,
                    data: char_data,
                },
            );
        }
        Ok(Self {
            name: match path.as_ref().to_str() {
                Some(name) => Some(name.to_string()),
                None => None,
            },
            height,
            len,
            data: char_map,
        })
    }
    pub fn defult() -> Self {
        Self {
            name: None,
            height: 0,
            len: 0,
            data: HashMap::new(),
        }
    }
    pub fn get<T: Display>(&self, st: T) -> Option<&SingleChar> {
        self.data.get(&st.to_string())
    }
    pub fn get_len(&self) -> usize {
        self.len
    }
}
#[derive(Debug, Clone)]
struct SingleChar {
    height: u32,
    width: u32,
    data: Vec<bool>,
}
/*
impl SingleChar {
    pub fn get_hw(&self)->(u32,u32){
        (self.height,self.width)
    }
}
*/
