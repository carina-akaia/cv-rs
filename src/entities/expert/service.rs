use {super::{mocks::*, model::*},
     dioxus::prelude::ScopeState,
     fermi::*};

static PROFILE_MOCK: Atom<Expert> = Atom(|_| {
	Expert { name: NAME_MOCK.into(),

	         roles: vec![
	                     "Senior Frontend Engineer".into(),
	                     "Full-stack Engineer".into(),
	                     "Software Architect".into(),
	                     "Junior Smart Contract Developer".into()
	],

	         avatar_url: AVATAR_URL_MOCK.into(),
	         banner_url: Some(BANNER_URL_MOCK.into()),

	         links: Links { website:  Some(WEBSITE_URL_MOCK.into()),
	                        linkedin: Some(LINKEDIN_URL_MOCK.into()),
	                        github:   Some(GITHUB_URL_MOCK.into()),
	                        telegram: Some(TELEGRAM_URL_MOCK.into()),
	                        x:        None, },

	         location: None,
	         status:   Some(STATUS_MOCK.into()),
	         contacts: Contacts { email: "cvo.akaia@gmail.com".into(), },
	         summary:  SUMMARY_MOCK.into(),

	         skills: vec![
	                      Skill { name: "Rust".into(),
	                              icon: None, },
	                      Skill { name: "TypeScript".into(),
	                              icon: None, },
	                      Skill { name: "JavaScript".into(),
	                              icon: None, },
	                      Skill { name: "HTML".into(),
	                              icon: None, },
	                      Skill { name: "CSS".into(),
	                              icon: None, },
	                      Skill { name: "NEAR Protocol".into(),
	                              icon: None, },
	                      Skill { name: "Clojure".into(),
	                              icon: None, },
	                      Skill { name: "ClojureScript".into(),
	                              icon: None, },
	],

	         experience: vec![Position { organization_name: "NEAR DevHub".into(),
	                                     roles:             vec!["Senior Frontend Engineer".into()],
	                                     term:              "Paid Open Source Contributor".into(),

	                                     start_date: 1625097600,
	                                     end_date:   None,
	                                     summary:    Some("Something about DevHub".into()), },], }
});

pub fn use_profile(cx: &ScopeState) -> &Expert {
	use_read(cx, &PROFILE_MOCK)
}
