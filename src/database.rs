use chrono::Utc;
use diesel::{prelude::*, result::Error};
use serenity::model::{guild::Guild, prelude::User};
use crate::{models::{Ban, NewBan, Mute, NewMute}, util::user_handle};
use crate::schema::{banlist,mutelist};

pub fn establish_connection() -> PgConnection {
	let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");
	PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", &database_url))
}

pub fn log_ban<'a>(user: &User, guild: Guild) -> Result<Ban, Error> {
	let conn = establish_connection();
	let new_ban = NewBan {
		user_id: &user.id.to_string(),
		server_id: &guild.id.to_string(),
		user_handle: &user_handle(user),
		date: Utc::now().naive_utc(),
	};
	diesel::insert_into(banlist::table)
		.values(&new_ban)
		.get_result(&conn)
}

pub fn log_unban<'a>(user: &User, guild: Guild) -> Result<Ban, Error> {
	let conn = establish_connection();
	diesel::delete(banlist::table)
		.filter(banlist::user_id.eq(user.id.to_string()))
		.filter(banlist::server_id.eq(guild.id.to_string()))
		.get_result(&conn)
}

pub fn log_mute<'a>(user: &User, guild: Guild) -> Result<Mute, Error> {
	let conn = establish_connection();
	let new_mute = NewMute {
		user_id: &user.id.to_string(),
		server_id: &guild.id.to_string(),
		user_handle: &user_handle(user),
		date: Utc::now().naive_utc(),
	};
	diesel::insert_into(mutelist::table)
		.values(&new_mute)
		.get_result(&conn)
}

pub fn log_unmute<'a>(user: &User, guild: Guild) -> Result<Mute, Error> {
	let conn = establish_connection();
	diesel::delete(mutelist::table)
		.filter(mutelist::user_id.eq(user.id.to_string()))
		.filter(mutelist::server_id.eq(guild.id.to_string()))
		.get_result(&conn)
}