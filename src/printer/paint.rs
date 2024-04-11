use image::RgbImage;
use std::{error::Error,fmt::Display};
use super::Printer;

impl Printer {
    pub fn draw<T:Display>(&self,centens:T,x:u32,y:u32,img:&mut RgbImage)->Result<(),Box<dyn Error>>{
        let s = centens.to_string();
        let s:Vec<_> = s.split("").collect();
        let mut offset_x = 0;
        let mut offset_y = 0;
        for i in s{
            if i == "" {
                continue;
            }
            if i == "\n"{
                offset_x = 0;
                offset_y += self.char_data.height*self.size+self.gap;
            }
            let len = self.draw_char(i, x+offset_x, y+offset_y, img)?;
            offset_x+=len as u32;
            offset_x+=self.gap;
        }
        Ok(())
    }
    pub fn draw_char(&self,st:&str,x:u32,y:u32,img:&mut RgbImage)->Result<u32,Box<dyn Error>>{
        if st == " "{
            return Ok(self.size*2);
        }
        let v = self.char_data.get(st);
        let v = match v {
            Some(t) => t,
            None => return Ok(0)
        };
        let mut it = v.data.clone().into_iter();
        for j in 0..v.height{
            for i in 0..v.width{
                if it.next().unwrap(){
                    if y < j * self.size{
                        continue;
                    }
                    self.draw_block(x + i * self.size, y - j * self.size, img)
                }
            }
        }
        Ok(v.width*self.size)
    }
    fn draw_block(&self,x:u32,y:u32,img:&mut RgbImage){
        let side_size = if (self.size/8)!=0{
            self.size/8
        } else {
            if self.size>2{
                1
            } else{
                0
            }
        };

        for i in 0..self.size{
            for j in 0..self.size{
                if i < side_size || i >= self.size-side_size||j < side_size || j >= self.size-side_size{
                    if x+i < img.width()&&y>=j{
                        img.put_pixel(x+i, y-j,self.color_out)
                    }
                    
                } else {
                    if x+i < img.width()&&y>=j{
                        img.put_pixel(x+i, y-j,self.color_in)
                    }
                }
            }
        }
    }
}