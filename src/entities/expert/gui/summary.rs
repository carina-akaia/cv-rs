use {crate::entities::expert::*,
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::*, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertSummaryProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertSummary(cx: Scope<ExpertSummaryProps>) -> Element {
	let ExpertSummaryProps { class } = cx.props;
	let model::Expert { summary, .. } = use_profile(cx);

	let root_class = class!(
		uno![cx; "flex-(~ col)", "gap-6"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		summary { class: root_class,
			h2 { class: uno![cx; "text-6", "font-600", "flex-(~)", "gap-2", "items-center", "m-0"],
				Icon {
					class: uno![cx; "fill-white"],
					width: 30,
					height: 30,
					icon: HiDocumentText
				}

				span { "Summary" }
			}

			p { class: uno![cx; "text-4", "m-0"], "{summary}" }
		}
	)
}
