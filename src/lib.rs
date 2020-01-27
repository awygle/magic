use byteorder::{ReadBytesExt, BigEndian};
use std::io;
use either::Either;

fn add_fp_colors(x :u32, y :u32) -> u32 {
    let working_x = x;
    let working_y = y;
    //let mut working_x = x & 0x7FFFFFFF;
    //let mut working_y = x & 0x7FFFFFFF;
    //if (x < 0) {
    //    working_x = -working_x;
    //}
    //if (y < 0) {
    //    working_y = -working_y;
    //}
    
    return working_x.wrapping_add(working_y);
}

// TODO don't make everything pub, write a constructor
#[derive(Debug)]
pub struct EdgeCoefficients {
    pub right_major: bool,
    pub level: u8,
    pub tile: u8,
    pub yl: u16,
    pub ym: u16,
    pub yh: u16,
    pub xl: u16,
    pub xl_frac: u16,
    pub dx_l_dy: u16,
    pub dx_l_dy_frac: u16,
    pub xh: u16,
    pub xh_frac: u16,
    pub dx_h_dy: u16,
    pub dx_h_dy_frac: u16,
    pub xm: u16,
    pub xm_frac: u16,
    pub dx_m_dy: u16,
    pub dx_m_dy_frac: u16,
}

impl EdgeCoefficients {
    pub fn yh(&self) -> u16 {
        self.yh
    }
    
    pub fn ym(&self) -> u16 {
        self.ym
    }
    
    pub fn yl(&self) -> u16 {
        self.yl
    }
    
    pub fn xh(&self) -> i32 {
        ((self.xh as u32) << 16 | self.xh_frac as u32) as i32
    }
    
    pub fn xm(&self) -> i32 {
        ((self.xm as u32) << 16 | self.xm_frac as u32) as i32
    }
    
    pub fn xl(&self) -> i32 {
        ((self.xl as u32) << 16 | self.xl_frac as u32) as i32
    }
    
    pub fn dx_h_dy(&self) -> i32 {
        ((self.dx_h_dy as u32) << 16 | self.dx_h_dy_frac as u32) as i32
    }
    
    pub fn dx_m_dy(&self) -> i32 {
        ((self.dx_m_dy as u32) << 16 | self.dx_m_dy_frac as u32) as i32
    }
    
    pub fn dx_l_dy(&self) -> i32 {
        ((self.dx_l_dy as u32) << 16 | self.dx_l_dy_frac as u32) as i32
    }
    
    pub fn read<T: io::Read>(reader: &mut T, value: u64) -> io::Result<EdgeCoefficients> {
        let right_major = value & 0x0080_0000_0000_0000 > 0;
        let level = ((value >> 51) as u8) & 0x07;
        let tile = ((value >> 48) as u8) & 0x07;
        let yl = ((value >> 32) as u16) & 0x3FFF;
        let ym = ((value >> 16) as u16) & 0x3FFF;
        let yh = (value as u16) & 0x3FFF;
        
        let data = reader.read_u64::<BigEndian>()?;
        let xl = (data >> 48) as u16;
        let xl_frac = (data >> 32) as u16;
        let dx_l_dy = (data >> 16) as u16;
        let dx_l_dy_frac = data as u16;
        
        let data = reader.read_u64::<BigEndian>()?;
        let xh = (data >> 48) as u16;
        let xh_frac = (data >> 32) as u16;
        let dx_h_dy = (data >> 16) as u16;
        let dx_h_dy_frac = data as u16;
        
        let data = reader.read_u64::<BigEndian>()?;
        let xm = (data >> 48) as u16;
        let xm_frac = (data >> 32) as u16;
        let dx_m_dy = (data >> 16) as u16;
        let dx_m_dy_frac = data as u16;
        
        Ok(EdgeCoefficients {
            right_major,
            level,
            tile,
            yl,
            ym,
            yh,
            xl,
            xl_frac,
            dx_l_dy,
            dx_l_dy_frac,
            xh,
            xh_frac,
            dx_h_dy,
            dx_h_dy_frac,
            xm,
            xm_frac,
            dx_m_dy,
            dx_m_dy_frac,
        })
    }
}

#[derive(Debug)]
pub struct ShadeCoefficients {
    red: u16,
    green: u16,
    blue: u16,
    alpha: u16,
    dr_dx: u16,
    dg_dx: u16,
    db_dx: u16,
    da_dx: u16,
    red_frac: u16,
    green_frac: u16,
    blue_frac: u16,
    alpha_frac: u16,
    dr_dx_frac: u16,
    dg_dx_frac: u16,
    db_dx_frac: u16,
    da_dx_frac: u16,
    dr_de: u16,
    dg_de: u16,
    db_de: u16,
    da_de: u16,
    dr_dy: u16,
    dg_dy: u16,
    db_dy: u16,
    da_dy: u16,
    dr_de_frac: u16,
    dg_de_frac: u16,
    db_de_frac: u16,
    da_de_frac: u16,
    dr_dy_frac: u16,
    dg_dy_frac: u16,
    db_dy_frac: u16,
    da_dy_frac: u16,
}

#[derive(Debug)]
pub struct TextureCoefficients {
    s: u16,
    t: u16,
    w: u16,
    ds_dx: u16,
    dt_dx: u16,
    dw_dx: u16,
    s_frac: u16,
    t_frac: u16,
    w_frac: u16,
    ds_dx_frac: u16,
    dt_dx_frac: u16,
    dw_dx_frac: u16,
    ds_de: u16,
    dt_de: u16,
    dw_de: u16,
    ds_dy: u16,
    dt_dy: u16,
    dw_dy: u16,
    ds_de_frac: u16,
    dt_de_frac: u16,
    dw_de_frac: u16,
    ds_dy_frac: u16,
    dt_dy_frac: u16,
    dw_dy_frac: u16,
}

fn whole2frac(x: u16, frac: u16) -> i32 {
    ((x as u32) << 16 | (frac as u32)) as i32
}

impl TextureCoefficients {
    pub fn s(&self) -> i32 {
        whole2frac(self.s, self.s_frac)
    }
    
    pub fn t(&self) -> i32 {
        whole2frac(self.t, self.t_frac)
    }
    
    pub fn w(&self) -> i32 {
        whole2frac(self.w, self.w_frac)
    }
    
    pub fn ds_dx(&self) -> i32 {
        whole2frac(self.ds_dx, self.ds_dx_frac)
    }
    
    pub fn dt_dx(&self) -> i32 {
        whole2frac(self.dt_dx, self.dt_dx_frac)
    }
    
    pub fn dw_dx(&self) -> i32 {
        whole2frac(self.dw_dx, self.dw_dx_frac)
    }
    
    pub fn ds_dy(&self) -> i32 {
        whole2frac(self.ds_dy, self.ds_dy_frac)
    }
    
    pub fn dt_dy(&self) -> i32 {
        whole2frac(self.dt_dy, self.dt_dy_frac)
    }
    
    pub fn dw_dy(&self) -> i32 {
        whole2frac(self.dw_dy, self.dw_dy_frac)
    }
    
    pub fn ds_de(&self) -> i32 {
        whole2frac(self.ds_de, self.ds_de_frac)
    }
    
    pub fn dt_de(&self) -> i32 {
        whole2frac(self.dt_de, self.dt_de_frac)
    }
    
    pub fn dw_de(&self) -> i32 {
        whole2frac(self.dw_de, self.dw_de_frac)
    }
    
}

impl ShadeCoefficients {
    pub fn red(&self) -> i32 {
        ((self.red as u32) << 16 | (self.red_frac as u32)) as i32
    }
    
    pub fn green(&self) -> i32 {
        ((self.green as u32) << 16 | (self.green_frac as u32)) as i32
    }
    
    pub fn blue(&self) -> i32 {
        ((self.blue as u32) << 16 | (self.blue_frac as u32)) as i32
    }
    
    pub fn alpha(&self) -> i32 {
        ((self.alpha as u32) << 16 | (self.alpha_frac as u32)) as i32
    }
    
    pub fn dr_dx(&self) -> i32 {
        ((self.dr_dx as u32) << 16 | (self.dr_dx_frac as u32)) as i32
    }
    
    pub fn dg_dx(&self) -> i32 {
        ((self.dg_dx as u32) << 16 | (self.dg_dx_frac as u32)) as i32
    }
    
    pub fn db_dx(&self) -> i32 {
        ((self.db_dx as u32) << 16 | (self.db_dx_frac as u32)) as i32
    }
    
    pub fn da_dx(&self) -> i32 {
        ((self.da_dx as u32) << 16 | (self.da_dx_frac as u32)) as i32
    }
    
    pub fn dr_dy(&self) -> i32 {
        ((self.dr_dy as u32) << 16 | (self.dr_dy_frac as u32)) as i32
    }
    
    pub fn dg_dy(&self) -> i32 {
        ((self.dg_dy as u32) << 16 | (self.dg_dy_frac as u32)) as i32
    }
    
    pub fn db_dy(&self) -> i32 {
        ((self.db_dy as u32) << 16 | (self.db_dy_frac as u32)) as i32
    }
    
    pub fn da_dy(&self) -> i32 {
        ((self.da_dy as u32) << 16 | (self.da_dy_frac as u32)) as i32
    }
    
    pub fn dr_de(&self) -> i32 {
        ((self.dr_de as u32) << 16 | (self.dr_de_frac as u32)) as i32
    }
    
    pub fn dg_de(&self) -> i32 {
        ((self.dg_de as u32) << 16 | (self.dg_de_frac as u32)) as i32
    }
    
    pub fn db_de(&self) -> i32 {
        ((self.db_de as u32) << 16 | (self.db_de_frac as u32)) as i32
    }
    
    pub fn da_de(&self) -> i32 {
        ((self.da_de as u32) << 16 | (self.da_de_frac as u32)) as i32
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub xl: u16,
    pub yl: u16,
    pub xh: u16,
    pub yh: u16,
}

#[derive(Debug)]
pub struct FillColor {
    pub color1: u32,
    pub color2: u32,
}

fn read_field_u8(value: u64, base: u8, size: u8) -> u8{
    ((value >> base) as u8) & ((1 << size) - 1)
}

#[derive(Debug, Copy, Clone)]
pub enum TextureFormat {
    RGBA,
    YUV,
    ColorIndex,
    IA,
    I,
}

impl TextureFormat {
    pub fn from_u8(x: u8) -> TextureFormat {
        match x {
            1 => TextureFormat::YUV,
            2 => TextureFormat::ColorIndex,
            3 => TextureFormat::IA,
            4 => TextureFormat::I,
            _ => TextureFormat::RGBA,
        }
    }
}

impl Default for TextureFormat {
    fn default() -> Self {
        TextureFormat::RGBA
    }
}

#[derive(Debug, Copy, Clone)]
pub enum TextureSize {
    Bit4,
    Bit8,
    Bit16,
    Bit32,
}

impl Default for TextureSize {
    fn default() -> Self {
        TextureSize::Bit4
    }
}

impl TextureSize {
    pub fn from_u8(x: u8) -> TextureSize {
        match x {
            1 => TextureSize::Bit8,
            2 => TextureSize::Bit16,
            3 => TextureSize::Bit32,
            _ => TextureSize::Bit4,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct ImageReference {
    pub format: TextureFormat,
    pub size: TextureSize,
    pub width: u16,
    pub addr: u32,
}

#[derive(Debug)]
pub struct CombineMode {
    sub_a_r_0: u8,
    mul_r_0: u8,
    sub_a_a_0: u8,
    mul_a_0: u8,
    sub_a_r_1: u8,
    mul_r_1: u8,
    sub_b_r_0: u8,
    sub_b_r_1: u8,
    sub_a_a_1: u8,
    mul_a_1: u8,
    add_r_0: u8,
    sub_b_a_0: u8,
    add_a_0: u8,
    add_r_1: u8,
    sub_b_a_1: u8,
    add_a_1: u8,
}

#[derive(Debug)]
pub enum RDPCommand {
    FillTriangle(EdgeCoefficients),
    ShadeTriangle(EdgeCoefficients, ShadeCoefficients),
    TextureTriangle(EdgeCoefficients, TextureCoefficients),
    FillRectangle(Rectangle),
    SetFillColor(FillColor),
    SetCombineMode(CombineMode),
    SetTextureImage(ImageReference),
    Nop,
}

pub trait ReadRdpCommands: io::Read where Self: std::marker::Sized {
    fn read_command(&mut self) -> io::Result<RDPCommand> {
        let value = self.read_u64::<BigEndian>()?;
        let opcode = ((value >> 56) & 0x3F) as u8;
        match opcode {
            0x00 => Ok(RDPCommand::Nop),
            0x3c => {
                let sub_a_r_0 = read_field_u8(value, 52, 4);
                let mul_r_0 = read_field_u8(value, 47, 5);
                let sub_a_a_0 = read_field_u8(value, 44, 3);
                let mul_a_0 = read_field_u8(value, 44, 3);
                let sub_a_r_1 = read_field_u8(value, 37, 4);
                let mul_r_1 = read_field_u8(value, 32, 5);
                let sub_b_r_0 = read_field_u8(value, 28, 4);
                let sub_b_r_1 = read_field_u8(value, 24, 4);
                let sub_a_a_1 = read_field_u8(value, 21, 3);
                let mul_a_1 = read_field_u8(value, 18, 3);
                let add_r_0 = read_field_u8(value, 15, 3);
                let sub_b_a_0 = read_field_u8(value, 12, 3);
                let add_a_0 = read_field_u8(value, 9, 3);
                let add_r_1 = read_field_u8(value, 6, 3);
                let sub_b_a_1 = read_field_u8(value, 3, 3);
                let add_a_1 = read_field_u8(value, 0, 3);
                Ok(RDPCommand::SetCombineMode(CombineMode {
                    sub_a_r_0,
                    mul_r_0,
                    sub_a_a_0,
                    mul_a_0,
                    sub_a_r_1,
                    mul_r_1,
                    sub_b_r_0,
                    sub_b_r_1,
                    sub_a_a_1,
                    mul_a_1,
                    add_r_0,
                    sub_b_a_0,
                    add_a_0,
                    add_r_1,
                    sub_b_a_1,
                    add_a_1,
                }))
            },
            0x37 => {
                // TODO handle both color modes
                // for now assume 32bpp
                Ok(RDPCommand::SetFillColor(FillColor {
                    color1: value as u32,
                    color2: value as u32,
                }))
            },
            0x3d => {
                // Set texture image
                println!("Set texture image command: {:#018X}", value);
                let format = ((value >> 53) & 0x7) as u8;
                let size = ((value >> 51) & 0x3) as u8;
                println!("Size is {}", size);
                let format = ((value >> 53) & 0x7) as u8;
                let width = ((value >> 32) & 0x3FF) as u16;
                let addr = (value & 0x03FFFFFF) as u32;
                Ok(RDPCommand::SetTextureImage(ImageReference {
                    format: TextureFormat::from_u8(format),
                    size: TextureSize::from_u8(size),
                    width,
                    addr,
                }))
            },
            0x36 => {
                let xl = ((value >> 44) & 0x0FFF) as u16;
                let yl = ((value >> 32) & 0x0FFF) as u16;
                let xh = ((value >> 12) & 0x0FFF) as u16;
                let yh = ((value >> 0) & 0x0FFF) as u16;
                Ok(RDPCommand::FillRectangle(Rectangle {
                    xl,
                    yl,
                    xh,
                    yh,
                }))
            },
            0x08 => {
                let edge_coefficients = EdgeCoefficients::read(self, value)?;
                
                Ok(RDPCommand::FillTriangle(edge_coefficients))
            },
            0x0c => {
                let edge_coefficients = EdgeCoefficients::read(self, value)?;
                
                let data = self.read_u64::<BigEndian>()?;
                let red = (data >> 48) as u16;
                let green = (data >> 32) as u16;
                let blue = (data >> 16) as u16;
                let alpha = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_dx = (data >> 48) as u16;
                let dg_dx = (data >> 32) as u16;
                let db_dx = (data >> 16) as u16;
                let da_dx = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let red_frac = (data >> 48) as u16;
                let green_frac = (data >> 32) as u16;
                let blue_frac = (data >> 16) as u16;
                let alpha_frac = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_dx_frac = (data >> 48) as u16;
                let dg_dx_frac = (data >> 32) as u16;
                let db_dx_frac = (data >> 16) as u16;
                let da_dx_frac = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_de = (data >> 48) as u16;
                let dg_de = (data >> 32) as u16;
                let db_de = (data >> 16) as u16;
                let da_de = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_dy = (data >> 48) as u16;
                let dg_dy = (data >> 32) as u16;
                let db_dy = (data >> 16) as u16;
                let da_dy = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_de_frac = (data >> 48) as u16;
                let dg_de_frac = (data >> 32) as u16;
                let db_de_frac = (data >> 16) as u16;
                let da_de_frac = (data >> 0) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let dr_dy_frac = (data >> 48) as u16;
                let dg_dy_frac = (data >> 32) as u16;
                let db_dy_frac = (data >> 16) as u16;
                let da_dy_frac = (data >> 0) as u16;
                
                let shade_coefficients = ShadeCoefficients {
                    red,
                    green,
                    blue,
                    alpha,
                    dr_dx,
                    dg_dx,
                    db_dx,
                    da_dx,
                    red_frac,
                    green_frac,
                    blue_frac,
                    alpha_frac,
                    dr_dx_frac,
                    dg_dx_frac,
                    db_dx_frac,
                    da_dx_frac,
                    dr_de,
                    dg_de,
                    db_de,
                    da_de,
                    dr_dy,
                    dg_dy,
                    db_dy,
                    da_dy,
                    dr_de_frac,
                    dg_de_frac,
                    db_de_frac,
                    da_de_frac,
                    dr_dy_frac,
                    dg_dy_frac,
                    db_dy_frac,
                    da_dy_frac,
                };
                
                Ok(RDPCommand::ShadeTriangle(edge_coefficients, shade_coefficients))
            },
            0x0a => {
                let edge_coefficients = EdgeCoefficients::read(self, value)?;
                
                let data = self.read_u64::<BigEndian>()?;
                let s = (data >> 48) as u16;
                let t = (data >> 32) as u16;
                let w = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_dx = (data >> 48) as u16;
                let dt_dx = (data >> 32) as u16;
                let dw_dx = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let s_frac = (data >> 48) as u16;
                let t_frac = (data >> 32) as u16;
                let w_frac = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_dx_frac = (data >> 48) as u16;
                let dt_dx_frac = (data >> 32) as u16;
                let dw_dx_frac = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_de = (data >> 48) as u16;
                let dt_de = (data >> 32) as u16;
                let dw_de = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_dy = (data >> 48) as u16;
                let dt_dy = (data >> 32) as u16;
                let dw_dy = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_de_frac = (data >> 48) as u16;
                let dt_de_frac = (data >> 32) as u16;
                let dw_de_frac = (data >> 16) as u16;
                
                let data = self.read_u64::<BigEndian>()?;
                let ds_dy_frac = (data >> 48) as u16;
                let dt_dy_frac = (data >> 32) as u16;
                let dw_dy_frac = (data >> 16) as u16;
                
                let texture_coefficients = TextureCoefficients {
                    s,
                    t,
                    w,
                    ds_dx,
                    dt_dx,
                    dw_dx,
                    s_frac,
                    t_frac,
                    w_frac,
                    ds_dx_frac,
                    dt_dx_frac,
                    dw_dx_frac,
                    ds_de,
                    dt_de,
                    dw_de,
                    ds_dy,
                    dt_dy,
                    dw_dy,
                    ds_de_frac,
                    dt_de_frac,
                    dw_de_frac,
                    ds_dy_frac,
                    dt_dy_frac,
                    dw_dy_frac,
                };
                
                Ok(RDPCommand::TextureTriangle(edge_coefficients, texture_coefficients))
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid opcode")),
        }
    }
}

impl<R: io::Read + ?Sized> ReadRdpCommands for R where R: std::marker::Sized {}

fn fill_between_pixels(fb: &mut [u8], x_start: usize, x_stop: usize, scanline: usize, color: [u8;4]) {
    for x in x_start..=x_stop {
        let index = ((scanline as usize) * 320 + (x as usize)) * 4;
        for i in 0..4 {
            fb[index+i] = color[i];
        }
    }
}

pub fn draw_fill_tri(fb: &mut [u8], coefficients: EdgeCoefficients, color: u32) {
    // clip YH to previous multiple of 4 - this is where XM and XH are defined
    let yh_whole_pixel = (coefficients.yh() / 4) * 4;
    // Walk down edges using slopes
    // (YH_whole, XH)
    let mut scanline = yh_whole_pixel / 4;
    let mut hx = coefficients.xh();
    let mut mx = coefficients.xm();
    
    while scanline < coefficients.ym() / 4 {
        let hx_pix = hx >> 16;
        let mx_pix = mx >> 16;
        let left = if coefficients.right_major { hx_pix } else { mx_pix };
        let right = if !coefficients.right_major { hx_pix } else { mx_pix };
        fill_between_pixels(fb, left as usize, right as usize, scanline as usize, color.to_be_bytes());
        hx += coefficients.dx_h_dy();
        mx += coefficients.dx_m_dy();
        scanline += 1;
    }
    
    let mut lx = coefficients.xl();
    
    while scanline < coefficients.yl() / 4 {
        let hx_pix = hx >> 16;
        let lx_pix = lx >> 16;
        let left = if coefficients.right_major { hx_pix } else { lx_pix };
        let right = if !coefficients.right_major { hx_pix } else { lx_pix };
        fill_between_pixels(fb, left as usize, right as usize, scanline as usize, color.to_be_bytes());
        hx = hx.wrapping_add(coefficients.dx_h_dy());
        lx = lx.wrapping_add(coefficients.dx_l_dy());
        scanline += 1;
    }
}

fn i32_to_u8_color(color: i32) -> u8 {
    if color < 0 {
        0
    } else {
        (color >> 16) as u8
    }
}

fn to_rgba(red: i32, green: i32, blue: i32, alpha: i32) -> u32 {
    let result = ((i32_to_u8_color(red) as u32) << 24) | ((i32_to_u8_color(green) as u32) << 16) | ((i32_to_u8_color(blue) as u32) << 8) | 0xFF as u32;
    result
}

pub fn draw_textured_tri(fb: &mut [u8], edge_coefficients: EdgeCoefficients, texture_coefficients: TextureCoefficients, texture: ImageReference, memory: &[u8]) {
    // clip YH to previous multiple of 4 - this is where XM and XH are defined
    let yh_whole_pixel = (edge_coefficients.yh() / 4) * 4;
    // Walk down edges using slopes
    // (YH_whole, XH)
    let mut scanline = yh_whole_pixel / 4;
    let mut hx = edge_coefficients.xh();
    let mut mx = edge_coefficients.xm();
    
    let ds_dx = if edge_coefficients.right_major { 
        texture_coefficients.ds_dx() 
    } else {
        -texture_coefficients.ds_dx()
    };
    let dt_dx = if edge_coefficients.right_major { 
        texture_coefficients.dt_dx() 
    } else {
        -texture_coefficients.dt_dx()
    };
    let dw_dx = if edge_coefficients.right_major { 
        texture_coefficients.dw_dx() 
    } else {
        -texture_coefficients.dw_dx()
    };
    
    // starting values
    let mut s = texture_coefficients.s();
    let mut t = texture_coefficients.t();
    let mut w = texture_coefficients.w();
    
    while scanline < edge_coefficients.ym() / 4 {
        let hx_pix = hx >> 16;
        let mx_pix = mx >> 16;
        
        // unnecessary muts due to iterator shenanigans
        let mut forward = hx_pix..mx_pix;
        let mut reverse = (mx_pix..hx_pix).rev();
        
        let line :&mut dyn Iterator<Item=_> = if edge_coefficients.right_major {
            &mut forward
        } else {
            &mut reverse
        };
        
        // edge stuff happens first, y stuff after?
        s += texture_coefficients.ds_de();
        t += texture_coefficients.dt_de();
        w += texture_coefficients.dw_de();
            
        let mut line_s = s;
        let mut line_t = t;
        let mut line_w = w;
        for x in line {
            println!("Texture coordinates for pixel {},{} are {:#010X}, {:#010X}, {:#010X}",
                     scanline, x, line_s, line_t, line_w);
            // Get color from texture here
            //let color = to_rgba(line_s, line_t, line_w, line_alpha);
            //let index = ((scanline as usize) * 320 + (x as usize)) * 4;
            //for i in 0..4 {
            //    fb[index+i] = color[i];
            //}
            line_s += ds_dx;
            line_t += dt_dx;
            line_w += dw_dx;
        }
        scanline += 1;
        
        s += texture_coefficients.ds_dy();
        t += texture_coefficients.dt_dy();
        w += texture_coefficients.dw_dy();
        
        hx += edge_coefficients.dx_h_dy();
        mx += edge_coefficients.dx_m_dy();
    }
    
    let mut lx = edge_coefficients.xl();
    
    while scanline < edge_coefficients.yl() / 4 {
        let hx_pix = hx >> 16;
        let lx_pix = lx >> 16;
        
        // unnecessary muts due to iterator shenanigans
        let mut forward = hx_pix..lx_pix;
        let mut reverse = (lx_pix..hx_pix).rev();
        
        let line :&mut dyn Iterator<Item=_> = if edge_coefficients.right_major {
            &mut forward
        } else {
            &mut reverse
        };
        
        // edge stuff happens first, y stuff after?
        s += texture_coefficients.ds_de();
        t += texture_coefficients.dt_de();
        w += texture_coefficients.dw_de();
            
        let mut line_s = s;
        let mut line_t = t;
        let mut line_w = w;
        for x in line {
            println!("Texture coordinates for pixel {},{} are {:#010X}, {:#010X}, {:#010X}",
                     scanline, x, line_s, line_t, line_w);
            // Get color from texture here
            //let color = to_rgba(line_s, line_t, line_w, line_alpha);
            //let color = color.to_be_bytes();
            //let index = ((scanline as usize) * 320 + (x as usize)) * 4;
            //for i in 0..4 {
            //    fb[index+i] = color[i];
            //}
            line_s += ds_dx;
            line_t += dt_dx;
            line_w += dw_dx;
        }
        scanline += 1;
        
        s += texture_coefficients.ds_dy();
        t += texture_coefficients.dt_dy();
        w += texture_coefficients.dw_dy();
        
        hx += edge_coefficients.dx_h_dy();
        lx += edge_coefficients.dx_l_dy();
    }
}

pub fn draw_shade_tri(fb: &mut [u8], edge_coefficients: EdgeCoefficients, shade_coefficients: ShadeCoefficients) {
    // clip YH to previous multiple of 4 - this is where XM and XH are defined
    let yh_whole_pixel = (edge_coefficients.yh() / 4) * 4;
    // Walk down edges using slopes
    // (YH_whole, XH)
    let mut scanline = yh_whole_pixel / 4;
    let mut hx = edge_coefficients.xh();
    let mut mx = edge_coefficients.xm();
    
    let dr_dx = if edge_coefficients.right_major { 
        shade_coefficients.dr_dx() 
    } else {
        -shade_coefficients.dr_dx()
    };
    let dg_dx = if edge_coefficients.right_major { 
        shade_coefficients.dg_dx() 
    } else {
        -shade_coefficients.dg_dx()
    };
    let db_dx = if edge_coefficients.right_major { 
        shade_coefficients.db_dx() 
    } else {
        -shade_coefficients.db_dx()
    };
    let da_dx = if edge_coefficients.right_major { 
        shade_coefficients.da_dx() 
    } else {
        -shade_coefficients.da_dx()
    };
    
    // starting values
    let mut red = shade_coefficients.red();
    let mut green = shade_coefficients.green();
    let mut blue = shade_coefficients.blue();
    let mut alpha = shade_coefficients.alpha();
    
    while scanline < edge_coefficients.ym() / 4 {
        let hx_pix = hx >> 16;
        let mx_pix = mx >> 16;
        
        // unnecessary muts due to iterator shenanigans
        let mut forward = hx_pix..mx_pix;
        let mut reverse = (mx_pix..hx_pix).rev();
        
        let line :&mut dyn Iterator<Item=_> = if edge_coefficients.right_major {
            &mut forward
        } else {
            &mut reverse
        };
        
        // edge stuff happens first, y stuff after?
        red += shade_coefficients.dr_de();
        green += shade_coefficients.dg_de();
        blue += shade_coefficients.db_de();
        alpha += shade_coefficients.da_de();
            
        let mut line_red = red;
        let mut line_green = green;
        let mut line_blue = blue;
        let mut line_alpha = alpha;
        for x in line {
            let color = to_rgba(line_red, line_green, line_blue, line_alpha);
            let color = color.to_be_bytes();
            let index = ((scanline as usize) * 320 + (x as usize)) * 4;
            for i in 0..4 {
                fb[index+i] = color[i];
            }
            line_red += dr_dx;
            line_green += dg_dx;
            line_blue += db_dx;
            line_alpha += da_dx;
        }
        scanline += 1;
        
        red += shade_coefficients.dr_dy();
        green += shade_coefficients.dg_dy();
        blue += shade_coefficients.db_dy();
        alpha += shade_coefficients.da_dy();
        
        hx += edge_coefficients.dx_h_dy();
        mx += edge_coefficients.dx_m_dy();
    }
    
    let mut lx = edge_coefficients.xl();
    
    while scanline < edge_coefficients.yl() / 4 {
        let hx_pix = hx >> 16;
        let lx_pix = lx >> 16;
        
        // unnecessary muts due to iterator shenanigans
        let mut forward = hx_pix..lx_pix;
        let mut reverse = (lx_pix..hx_pix).rev();
        
        let line :&mut dyn Iterator<Item=_> = if edge_coefficients.right_major {
            &mut forward
        } else {
            &mut reverse
        };
        
        // edge stuff happens first, y stuff after?
        red += shade_coefficients.dr_de();
        green += shade_coefficients.dg_de();
        blue += shade_coefficients.db_de();
        alpha += shade_coefficients.da_de();
            
        let mut line_red = red;
        let mut line_green = green;
        let mut line_blue = blue;
        let mut line_alpha = alpha;
        for x in line {
            let color = to_rgba(line_red, line_green, line_blue, line_alpha);
            let color = color.to_be_bytes();
            let index = ((scanline as usize) * 320 + (x as usize)) * 4;
            for i in 0..4 {
                fb[index+i] = color[i];
            }
            line_red += dr_dx;
            line_green += dg_dx;
            line_blue += db_dx;
            line_alpha += da_dx;
        }
        scanline += 1;
        
        red += shade_coefficients.dr_dy();
        green += shade_coefficients.dg_dy();
        blue += shade_coefficients.db_dy();
        alpha += shade_coefficients.da_dy();
        
        hx += edge_coefficients.dx_h_dy();
        lx += edge_coefficients.dx_l_dy();
    }
}

pub fn draw_fill_rect(fb: &mut [u8], rectangle: Rectangle, color: u32) {
    let xh_pix = rectangle.xh >> 2;
    let yh_pix = rectangle.yh >> 2;
    let xl_pix = rectangle.xl >> 2;
    let yl_pix = rectangle.yl >> 2;
    
    let fake_color :u32 = 0x55AA55FF;
    for y in yh_pix..=yl_pix {
        fill_between_pixels(fb, xh_pix as usize, xl_pix as usize, y as usize, fake_color.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use super::*;
    
    #[test]
    fn parse_nop() {
        let data = [0u8; 8];
        let mut reader = BufReader::new(&data[..]);
        let command :RDPCommand = reader.read_command().unwrap();
        match command {
            RDPCommand::Nop => {},
            _ => panic!("wrong command"),
        }
    }
    
    #[test]
    fn parse_invalid() {
        let data = [1u8; 8];
        let mut reader = BufReader::new(&data[..]);
        let command :io::Result<RDPCommand> = reader.read_command();
        assert!(command.is_err())
    }
}
