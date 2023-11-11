use {crate::{core::uikit::*, entities::expert::*},
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::*, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertSkillsetProps {
	#[props(into)]
	class: Option<String>,
}

pub fn ExpertSkillset(cx: Scope<ExpertSkillsetProps>) -> Element {
	let ExpertSkillsetProps { class } = cx.props;
	let model::Expert { skills, .. } = use_profile(cx);

	let skill_badges = skills.into_iter()
	                         .map(|skill| rsx!(Badge { text: "{skill.name}", }));

	let root_class = class!(
		uno![cx; "flex-(~ col)", "gap-6"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		aside { class: root_class,
			h2 { class: uno![cx; "text-6", "font-600", "flex-(~)", "gap-2", "items-center", "m-0"],
				Icon { class: uno![cx; "fill-white"], width: 30, height: 30, icon: HiStar }
				span { "Skills" }
			}

			div { class: uno![cx; "flex-(~ wrap)", "gap-3"], skill_badges }
		}
	)
}
