use {super::position::*,
     crate::entities::expert::*,
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::HiBookmark, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertExperienceProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertExperience(cx: Scope<ExpertExperienceProps>) -> Element {
	let ExpertExperienceProps { class } = cx.props;
	let model::Expert { experience, .. } = use_profile(cx);

	let root_class = class!(
		uno![cx; "flex-(~ col)", "gap-6"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		section { class: root_class,
			h2 { class: uno![cx; "text-6", "font-600", "flex-(~)", "gap-2", "items-center", "m-0"],
				Icon { class: uno![cx; "fill-white"], width: 30, height: 30, icon: HiBookmark }
				span { "Experience" }
			}

			experience.into_iter().rev().map(|position| {rsx! { ExpertPosition { data: position.to_owned() } }})
		}
	)
}
