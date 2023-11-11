use {crate::entities::expert::{model::Position, *},
     dioxus::prelude::*,
     dioxus_class::prelude::*,
     dioxus_free_icons::{icons::hi_outline_icons::HiOfficeBuilding, Icon}};

#[derive(PartialEq, Props)]
pub struct ExpertPositionProps {
	#[props(into)]
	class: Option<String>,
	data:  model::Position,
}

pub fn ExpertPosition(cx: Scope<ExpertPositionProps>) -> Element {
	let ExpertPositionProps { class, data } = cx.props;

	let Position { organization_name,
	               summary,
	               .. } = data;

	let summary_string = summary.to_owned().unwrap_or("".into());

	let root_class = class!(
		uno![cx; "flex-(~)", "gap-3"]
		class.to_owned().unwrap_or("".into())
	);

	render!(
		article { class: root_class,
			Icon {
				class: uno![cx; "fill-white"],
				width: 30,
				height: 30,
				icon: HiOfficeBuilding
			}

			div { class: uno![cx; "flex-(~ col)", "gap-2"],
				h3 { class: uno![cx; "m-0", "text-5", "font-medium"], format!("{}", organization_name) }
				p { class: uno![cx; "m-0", "text-4", "hidden" => summary_string.len() == 0], format!("{}", summary_string) }
			}
		}
	)
}
