use {dioxus::prelude::*, dioxus_class::prelude::*};

#[derive(Props)]
pub struct BadgeProps<'a> {
	class:    Option<&'a str>,
	text:     &'a str,
	children: Element<'a>,
}

pub fn Badge<'a>(cx: Scope<'a, BadgeProps<'a>>) -> Element {
	let BadgeProps { children,
	                 class,
	                 text, } = cx.props;

	let root_class = class!(
		uno![cx; "text-(current 4)", "bg-[#29132e]", "rounded-4", "p-(x-3 y-1)", "print-p-0"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		span { class: root_class, children, "{text}" }
	)
}
