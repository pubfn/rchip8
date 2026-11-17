mod cpu;
use crate::hardware::display::Framebuffer;
use crate::hardware::keyboard::Keyboard;
use raylib::ffi;
use std::ffi::CString;
mod hardware;
use crate::cpu::Cpu;
use crate::hardware::display::Display;
use crate::hardware::fontset::Chip8Font;
use raylib::prelude::*;
use std::env;
use std::fs;

const INSTRUCTIONS_PER_FRAME: usize = 9;
const CELL_SIZE: usize = 15;
const SOUND_BYTES: &[u8] = include_bytes!("sound/perfection.wav");

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2
    {
        panic!("usage: chip8 <rom>");
    }

    let rom = fs::read(&args[1]).expect("failed to read rom");

    unsafe {
        ffi::InitAudioDevice();
    };

    let mut cpu = Cpu::new(&rom, Chip8Font::get());
    let mut display = Framebuffer::new();

    let (mut rl, thread) = raylib::init().size(960, 480).build();

    rl.set_target_fps(60);

    let path = "temp.wav";
    fs::write(path, SOUND_BYTES).unwrap();

    let c_path = CString::new(path).unwrap();
    let sound = unsafe { ffi::LoadSound(c_path.as_ptr()) };

    while !rl.window_should_close()
    {
        let key = Keyboard::get_key(&rl);
        for _ in 0..INSTRUCTIONS_PER_FRAME
        {
            cpu.execute(&mut display, key);
        }

        unsafe {
            if cpu.get_sound_timer() > 0
            {
                if ffi::IsSoundPlaying(sound) == false
                {
                    ffi::PlaySound(sound);
                }
            }
            else
            {
                ffi::StopSound(sound);
            }
        }

        cpu.timers_step();
        draw_screen(&display, &mut rl, &thread);
    }
}

fn draw_screen(display: &Framebuffer, rl: &mut RaylibHandle, thread: &RaylibThread)
{
    rl.draw(thread, |mut d| {
        for i in 0..32
        {
            for j in 0..64
            {
                let color = if display.get_pixel(j, i)
                {
                    Color::WHITE
                }
                else
                {
                    Color::BLACK
                };
                d.draw_rectangle(
                    (j * CELL_SIZE) as i32,
                    (i * CELL_SIZE) as i32,
                    CELL_SIZE as i32,
                    CELL_SIZE as i32,
                    color,
                );
            }
        }
    })
}
