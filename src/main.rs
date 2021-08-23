use crossterm::cursor::Hide;
use crossterm::cursor::Show;
use crossterm::event::KeyCode;
use crossterm::execute;
use crossterm::style::ResetColor;
use crossterm::terminal::size;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use rand::Rng;
use std::io::{self};
use std::sync::mpsc;
use std::thread;
use std::{thread::sleep, time};

mod cell;
mod event;
mod world;

fn main() {
    let a = size().unwrap();
    let mut world = world::World::new(100, 150, (a.1 - 1).into(), (a.0 / 2).into());

    for _ in 0..2500 {
        world.set_cell_state(
            rand::thread_rng().gen_range(0..100),
            rand::thread_rng().gen_range(0..100),
        );
    }
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Hide).unwrap();
    let mut handle = io::BufWriter::new(stdout);

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        match event::get_key_code() {
            Some(key_code) => {
                tx.send(key_code).unwrap();
            }
            None => {}
        }
    });

    loop {
        let received = rx.try_recv();

        match received {
            Ok(key_code) => match key_code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Char('e') => {
                    world.world_pointer = world.print_point;
                    // world.world_pointer.0 += 25;
                    // world.world_pointer.1 += 50;
                    loop {
                        world.world_static = false;
                        world.print(&mut handle);
                        let key = rx.recv().unwrap();
                        match key {
                            KeyCode::Char('e') => {
                                world.world_static = true;
                                break;
                            }
                            _ => {
                                world.edit_world(key, &mut handle, true);
                            }
                        }
                    }
                }
                KeyCode::Char('s') => {
                    world.calculus_speed += 10;
                    if world.calculus_speed >= 100 {
                        world.calculus_speed = 10;
                    }
                }

                _ => {
                    world.edit_world(key_code, &mut handle, false);
                }
            },
            Err(_) => {}
        }

        world.update_world(&mut handle);

        sleep(time::Duration::from_millis(world.calculus_speed));
    }
    execute!(io::stdout(), ResetColor, Show, LeaveAlternateScreen).unwrap();
}
