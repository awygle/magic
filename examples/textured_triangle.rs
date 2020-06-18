use winit::{
    event::{
        Event,
        WindowEvent,
        KeyboardInput,
        VirtualKeyCode,
        ElementState,
    },
    event_loop::{
        EventLoop,
        ControlFlow,
    },
    dpi::LogicalSize,
    window::Window,
};
use imgui::*;
use imgui_wgpu::Renderer;
use imgui_winit_support;
use std::time::Instant;
use wgpu;
use std::fs;
use std::io::ErrorKind::UnexpectedEof;
use std::io::Read;
use std::convert::TryInto;
use byteorder::{ReadBytesExt, BigEndian};

use magic;
use magic::ReadRdpCommands;

pub trait FrameBufferExt {
    fn framebuffer(&self, fb: &[u8], width: u32, height: u32, renderer: &mut Renderer, device: &wgpu::Device, queue: &mut wgpu::Queue, texture_id: TextureId);
}

impl FrameBufferExt for Ui<'_> {
    fn framebuffer(&self, fb: &[u8], width: u32, height: u32, renderer: &mut Renderer, device: &wgpu::Device, queue: &mut wgpu::Queue, texture_id: TextureId) {
        renderer.update_texture(device, queue, fb, width, height, texture_id);
        
        Image::new(texture_id, [width as f32, height as f32]).build(self);
    }
}

fn get_image(memory: Vec<u8>, imageref: magic::ImageReference) -> Vec<u32> {
    let address = imageref.addr as usize;
    println!("Looking at address {:#010X} for image {:#?}", address, imageref);
    for i in (0..=(imageref.width as usize)).map(|x| address + x * 4) {
        let value = ((memory[i] as u32) << 24) | ((memory[i + 1] as u32) << 16) |
            ((memory[i + 2] as u32) << 8) | ((memory[i + 3] as u32));
        println!("Pixel {} is {:#010X}", i, value);
    }
    panic!();
    Vec::new()
}

fn main() {

    // Set up window and GPU    
    let event_loop = EventLoop::new();
    let (window, mut size, surface, hidpi_factor) = {
        let version = env!("CARGO_PKG_VERSION");

        let window = Window::new(&event_loop).unwrap();
        window.set_inner_size(LogicalSize { width: 1280.0, height: 720.0 });
        window.set_title(&format!("imgui-wgpu {}", version));
        let hidpi_factor = window.hidpi_factor();
        let size = window
            .inner_size()
            .to_physical(hidpi_factor);

        let surface = wgpu::Surface::create(&window);

        (window, size, surface, hidpi_factor)
    };

    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
        backends: wgpu::BackendBit::PRIMARY,
    }).unwrap();

    let (mut device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    // Set up swap chain
    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8Unorm,
        width: size.width as u32,
        height: size.height as u32,
        present_mode: wgpu::PresentMode::NoVsync,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    // Set up dear imgui
    let mut imgui = imgui::Context::create();
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), &window, imgui_winit_support::HiDpiMode::Default);
    imgui.set_ini_filename(None);

    let font_size = (13.0 * hidpi_factor) as f32;
    imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

    imgui.fonts().add_font(&[
        FontSource::DefaultFontData {
            config: Some(imgui::FontConfig {
                oversample_h: 1,
                pixel_snap_h: true,
                size_pixels: font_size,
                ..Default::default()
            })
        }
    ]);

    //
    // Set up dear imgui wgpu renderer
    // 
    let clear_color = wgpu::Color { r: 0.1, g: 0.2, b: 0.3, a: 1.0 };
    let mut renderer = Renderer::new(&mut imgui, &device, &mut queue, sc_desc.format, Some(clear_color));

    let mut last_frame = Instant::now();
    
    // Set up N64 texture
    let (width, height) = (320u32, 240u32);
    let mut fb_bytes = Vec::with_capacity(320*240*4);
    fb_bytes.resize_with(320*240*4, || 0xFF);
    let fb_tex_id = renderer.upload_texture(&device, &mut queue, &fb_bytes, width, height);
    
    // Read in input file
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open("texturelist.bin").expect("couldn't find input file");
    
    let num_bytes :usize = (file.metadata().expect("couldn't get file data").len()).try_into().unwrap();
    let mut command_bytes :Vec<u8> = Vec::with_capacity(num_bytes);
    command_bytes.resize_with(num_bytes, || 0);
    file.read(&mut command_bytes).expect("not enough commands");
    assert!(command_bytes.len() == num_bytes);
    
    // Read in texture file
    let mut texfile = fs::OpenOptions::new()
        .read(true)
        .open("Cycle1TextureTriangle32BPPRGBA32B320X240.N64").expect("couldn't find input file");
    let num_bytes :usize = (texfile.metadata().expect("couldn't get file data").len()).try_into().unwrap();
    let mut texture_bytes :Vec<u8> = Vec::with_capacity(num_bytes);
    texture_bytes.resize_with(num_bytes, || 0);
    texfile.read(&mut texture_bytes).expect("not enough commands");
    assert!(texture_bytes.len() == num_bytes);
    
    // build RDP struct
    let mut rdp = magic::RDP { 
        texture: magic::ImageReference::default(),
        tiles: [magic::Tile::default(); 8],
        texmem: [0; 4096],
        active_tile: 0,
    };

    // Event loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = if cfg!(feature = "metal-auto-capture") {
            ControlFlow::Exit
        } else {
            ControlFlow::Poll
        };
        let mut seen = false;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                size = window
                    .inner_size()
                    .to_physical(hidpi_factor);

                sc_desc = wgpu::SwapChainDescriptor {
                    usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                    format: wgpu::TextureFormat::Bgra8Unorm,
                    width: size.width as u32,
                    height: size.height as u32,
                    present_mode: wgpu::PresentMode::NoVsync
                };

                swap_chain = device.create_swap_chain(&surface, &sc_desc);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput {
                    input: KeyboardInput {
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        state: ElementState::Pressed,
                        ..
                    },
                    ..
                },
                ..
            } |
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            },
            Event::EventsCleared => {
                let now = Instant::now();
                last_frame = now;

                let frame = swap_chain.get_next_texture();
                platform.prepare_frame(imgui.io_mut(), &window)
                    .expect("Failed to prepare frame");
                let ui = imgui.frame();
                
                // draw triangles
                let mut command_slice = &command_bytes[..];
                let mut color = 0;
                loop {
                    let command_option: std::io::Result<magic::RDPCommand> = command_slice.read_command();
                    if let Ok(command) = command_option {
                        match command { 
                            magic::RDPCommand::FillTriangle(coefficients) =>  {
                                rdp.draw_fill_tri(&mut fb_bytes, coefficients, color);
                            },
                            magic::RDPCommand::FillRectangle(rectangle) => {
                                rdp.draw_fill_rect(&mut fb_bytes, rectangle, color);
                            },
                            magic::RDPCommand::TextureTriangle(coefficients, textures) =>  {
                                if !seen {
                                    println!("Drawing a textured triangle with these coefficients: {:#X?}\n{:#X?}", coefficients, textures);
                                    rdp.draw_textured_tri(&mut fb_bytes, coefficients, textures);
                                    //seen = true;
                                }
                            },
                            magic::RDPCommand::SetFillColor(colors) => {
                                //println!("Setting color to {:#X}", colors.color1);
                                color = colors.color1;
                            },
                            magic::RDPCommand::SetTextureImage(imageref) => {
                                rdp.texture = imageref;
                            },
                            magic::RDPCommand::SetTile(tile) => {
                                println!("Setting active tile to {}", tile.tile);
                                rdp.active_tile = tile.tile;
                                rdp.tiles[rdp.active_tile as usize] = tile
                            },
                            magic::RDPCommand::LoadTile { tile, sl, sh, tl, th } => {
                                // active tile tells us tmem base address in 64bit units
                                let base_addr :usize = rdp.tiles[tile as usize].tmem_addr as usize / 8;
                                let linewidth :usize = rdp.tiles[tile as usize].line_width as usize * 8;
                                // TODO handle the way pixel formats mess up line width
                                // calculations better
                                let linewidth = linewidth / 2;
                                let sl_usize = (sl >> 2) as usize;
                                let sh_usize = (sh >> 2) as usize;
                                let tl_usize = (tl >> 2) as usize;
                                let th_usize = (th >> 2) as usize;
                                
                                for y in tl_usize..=th_usize {
                                    for x in sl_usize..=sh_usize {
                                        let offset = (x + (y * linewidth)) * 4;
                                        let src_offset = (x + (y * ((rdp.texture.width as usize) + 1))) * 4;
                                        //println!("Copying from {}, {} (address {:#X})", x, y, (rdp.texture.addr as usize) + src_offset);
                                        //println!("Copying to {}, {} (address {:#X})", x, y, base_addr + offset);
                                        for i in 0..4 {
                                            rdp.texmem[base_addr + offset + i] = 
                                                texture_bytes[(rdp.texture.addr as usize) + src_offset + i];
                                        }
                                    }
                                }
                                //panic!();
                                
                                //println!("Texmem now contains:");
                                //for x in sl_usize..=sh_usize {
                                //    for y in tl_usize..=th_usize {
                                //        let offset = x + (y * linewidth);
                                //        for i in 0..4 {
                                //        println!("{:#04X}", rdp.texmem[base_addr + offset + i]);
                                //        }
                                //    }
                                //}
                            }
                            magic::RDPCommand::Nop => {},
                            _ => {
                                println!("Unknown command: {:?}", command);
                            }
                        }
                    }
                    else {
                        if command_option.unwrap_err().kind() == UnexpectedEof {
                            break;
                        }
                    }
                }
                
                {
                    let window = imgui::Window::new(im_str!("Hello world"));
                    window
                        .size([400.0, 600.0], Condition::FirstUseEver)
                        .build(&ui, || {
                            ui.text(im_str!("Hello textures!"));
                            ui.framebuffer(&fb_bytes, width, height, &mut renderer, &device, &mut queue, fb_tex_id);
                        });
                }

                let mut encoder: wgpu::CommandEncoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });

                platform.prepare_render(&ui, &window);
                renderer
                    .render(ui, &mut device, &mut encoder, &frame.view)
                    .expect("Rendering failed");

                queue.submit(&[encoder.finish()]);
            },
            _ => (),
        }

        platform.handle_event(imgui.io_mut(), &window, &event);
    });
}
