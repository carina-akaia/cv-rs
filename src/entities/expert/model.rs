use near_sdk::Timestamp;

pub struct Expert {
	pub name:       Name,
	pub roles:      Vec<Role>,
	pub avatar_url: URL,
	pub banner_url: Option<URL>,
	pub contacts:   Contacts,
	pub links:      Links,
	pub location:   Option<Location>,
	pub status:     Option<Status>,
	pub summary:    Summary,
	pub skills:     Vec<Skill>,
	pub experience: Vec<Position>,
}

pub type Name = String;
pub type Role = String;
pub type URL = String;

#[derive(Clone)]
pub struct Contacts {
	pub email: Email,
}

pub type Email = String;

#[derive(Clone)]
pub struct Links {
	pub website:  Option<Link>,
	pub linkedin: Option<Link>,
	pub github:   Option<Link>,
	pub telegram: Option<Link>,
	pub x:        Option<Link>,
}

pub type Link = String;
pub type Location = String;
pub type Status = String;
pub type Summary = String;

#[derive(Clone)]
pub struct Skill {
	pub name: String,
	pub icon: Option<String>,
}

#[derive(Clone, PartialEq)]
pub struct Position {
	pub organization_name: OrganizationName,
	pub roles:             Vec<Role>,
	pub term:              String,
	pub start_date:        Timestamp,
	pub end_date:          Option<Timestamp>,
	pub summary:           Option<String>,
}

pub type OrganizationName = String;
