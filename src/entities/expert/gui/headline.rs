use {crate::{core::uikit::*,
             entities::expert::{model::Contacts, *}},
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::fa_brands_icons::*, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertHeadlineProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertHeadline(cx: Scope<ExpertHeadlineProps>) -> Element {
	let ExpertHeadlineProps { class } = cx.props;

	let Expert { name,
	             roles,
	             avatar_url,
	             banner_url,
	             status,
	             contacts: Contacts { email },
	             links,
	             .. } = use_profile(cx);

	let status_string = status.to_owned().unwrap_or("".into());
	let role_list = roles.iter().map(|role| rsx!(Badge { text: role }));

	let website_link = if let Some(link) = &links.website {
		rsx!(
			li { Link { address: link, label: "NEAR Social", with_icon: true } }
		)
	} else {
		rsx!(Element::None)
	};

	let linkedin_link = if let Some(link) = &links.linkedin {
		rsx!(
			li {
				Link {
					address: link,
					label: "LinkedIn",
					with_icon: true,
					custom_icon: render! {
			Icon { class : uno![cx; "fill-current"], width : 24, height : 24, icon : FaLinkedin }
		}
				}
			}
		)
	} else {
		rsx!(Element::None)
	};

	let root_class = class!(
		uno![cx;
			"flex-(~ col)",
			"justify-center",
			"items-center",
			"gap-8",
			"border-(gray-5)",
		]

		class.to_owned().unwrap_or("".into())
	);

	let root_style = format!(
	                         "&:before {{ background-image: url({:?}) }};",
	                         banner_url.to_owned().unwrap_or("".to_owned())
	);

	render!(
		header { class: root_class, style: "{root_style}",
			div { class: uno![cx; "flex-(~)", "justify-center", "items-center", "gap-8"],
				div { class: uno![cx; "flex-(~ col)", "items-end", "gap-4", "line-height-none"],
					div { class: uno![cx; "flex-(~ col)", "items-end", "gap-1"],
						h1 { class: uno![cx; "m-0", "text-9", "font-600"], format!("{}", name) }
						span { class: uno![cx; "m-0", "text-(green-5)", "hidden" => status_string.len() == 0, "print-hidden"], format!("{}", status_string) }
					}

					Link { address: "mailto:{email}", class: uno![cx; "text-5"] }
				}

				img {
					class: uno![cx; "rounded-full", "w-36", "h-36"],
					// Prevent the image from being stretched before the class is applied.
					width: "9rem",
					height: "9rem",
					src: "{avatar_url}"
				}

				ul { class: uno![cx; "flex-(~ col)", "items-start", "gap-3", "list-none", "m-0", "p-0"], website_link, linkedin_link }
			}

			ul { class: uno![cx; "flex-(~ wrap)", "gap-4", "print-gap-6", "p-0", "m-0", "list-none"], role_list }
		}
	)
}
