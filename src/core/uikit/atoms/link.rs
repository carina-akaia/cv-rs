use {dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_solid_icons::*, Icon},
     regex::Regex,
     url::*};

#[derive(Props)]
pub struct LinkProps<'a> {
	address:     &'a str,
	#[props(default = Element::None)]
	custom_icon: Element<'a>,
	#[props(default = "")]
	class:       &'a str,
	label:       Option<&'a str>,
	#[props(default = false)]
	with_icon:   bool,
}

pub fn Link<'a>(cx: Scope<'a, LinkProps<'a>>) -> Element {
	let LinkProps { address,
	                class,
	                custom_icon,
	                label,
	                with_icon,
	                .. } = cx.props;

	let _url = Url::parse(address);

	let icon = if with_icon.to_owned() {
		if custom_icon.is_some() {
			rsx! {custom_icon}
		} else {
			rsx! { Icon { class: uno![cx; "fill-current"], icon: HiLink, width: 24, height: 24 } }
		}
	} else {
		rsx! {Element::None}
	};

	let address_shorthand = Regex::new(r"^(https?|mailto):(//)?").unwrap()
	                                                             .replace(address, "")
	                                                             .into_owned();

	let root_class = class!(
		uno![cx;
			"flex-(~)",
			"items-center",
			"gap-2",
			"text-(current 4)",
			"decoration-dotted",
			"print-decoration-none",
			"hover:decoration-solid",
			"m-0",
		]

		class.to_owned()
	);

	render!(
		a { class: root_class, href: address.to_owned(),
			icon,
			span { class: uno![cx; "block", "print-hidden"], label.unwrap_or(&address_shorthand) }
			span { class: uno![cx; "hidden", "print-block"], address_shorthand }
		}
	)
}
