#![allow(non_snake_case)]

mod core;
mod entities;
mod screens;
mod widgets;

#[macro_use]
extern crate unocss_classes;

fn main() {
	dioxus_web::launch(widgets::ExpertsApp);
}
