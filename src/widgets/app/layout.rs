use {dioxus::prelude::*, dioxus_class::prelude::*};

#[derive(Props)]
pub struct AppLayoutProps<'a> {
	class:    Option<&'a str>,
	children: Element<'a>,
}

pub fn AppLayout<'a>(cx: Scope<'a, AppLayoutProps<'a>>) -> Element {
	let AppLayoutProps { children, class } = cx.props;

	let root_class = class!(
		uno![cx; "h-full", "flex-(~ col)", "bg-black", "print-bg-white", "text-white", "print-text-black"]
		class.to_owned().unwrap_or("".into())
	);

	let nav_class = uno![cx; "flex-(~)", "items-center", "w-full", "h-16", "print-hidden"];

	let main_class = uno![
		cx; "w-full", "h-full", "flex-(~ col)", "items-center", "justify-center",
		"print-justify-start"
	];

	render!(
		div { class: root_class,
			nav { class: nav_class, span { class: uno![cx; "px-3", "text-(7)"], "AKAIA Experts" } }
			main { class: main_class, children }
		}
	)
}
