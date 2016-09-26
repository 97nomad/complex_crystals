extern crate sdl2;
extern crate sdl2_image;
extern crate sdl2_ttf;
extern crate rand;
extern crate rustc_serialize;
extern crate hyper;

mod views;
mod phi;
mod network;

fn main() {
    ::phi::spawn("Complex Crystal Client",
                 |phi| Box::new(::views::main_menu::MainMenuView::new(phi)));
}