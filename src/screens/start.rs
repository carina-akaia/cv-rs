use {crate::entities::expert::*, dioxus::prelude::*};

pub fn StartScreen(cx: Scope) -> Element {
	let root_class = uno![cx;
		"bg-[#29132e]",
		"bg-op-70",
		"rounded-4",
		"grid-(~ cols-6 rows-auto)",
		"items-baseline",
		"pb-1",
		"print-pb-0",
		"max-w-[1024px]",
		"print-max-w-full",
	];

	let watermark_class = uno![cx;
		"absolute", "justify-self-end", "text-(2)", "opacity-[0.2]", "hidden", "print-block"
	];

	render!(
		div { class: root_class,
			ExpertHeadline { class: uno![cx; "col-span-6", "row-span-4", "p-9", "print-(p-0 pb-8)"] }
			ExpertSkillset { class: uno![cx; "col-span-2", "row-span-auto", "p-8", "print-p-0", "pr-0"] }

			div { class: uno![cx; "col-span-4", "row-span-auto", "flex-(~ col)", "gap-8", "p-4"],
				ExpertSummary { class: uno![cx; "p-4", "print-p-0"] }
				ExpertExperience { class: uno![cx; "p-4", "print-p-0"] }
			}

			span { class: watermark_class, "Made in AKAIA" }
		}
	)
}
