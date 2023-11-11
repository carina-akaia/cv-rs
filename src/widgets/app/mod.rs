mod layout;
mod navigation;

use {self::{layout::*, navigation::*},
     dioxus::prelude::*,
     dioxus_router::prelude::*,
     fermi::use_init_atom_root};

pub fn ExpertsApp(cx: Scope) -> Element {
	use_init_atom_root(cx);

	render! {
		AppLayout { Router::<AppRoute> {} }
	}
}
