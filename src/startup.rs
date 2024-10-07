use crate::routes::health_check;
use rocket::fairing::AdHoc;
use rocket::{post, routes, Build, Rocket};
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket_sync_db_pools::{database, diesel};
use crate::task::Member;

#[database("bibimbap")]
pub struct DbConn(diesel::PgConnection);

#[post("/", data = "<member_form>")]
async fn new(member_form: Form<Member>, conn: DbConn) -> Flash<Redirect> {
    let member = member_form.into_inner();

    if member.id.is_empty() {
        Flash::error(Redirect::to("/"), "ID cannot be empty.")
    } else if let Err(e) = Task::insert(member, &conn).await {
        error!("DB insertion error: {e}");
        Flash::error(Redirect::to("/"), "Member could not be inserted due to an internal error.")
    } else {
        Flash::success(Redirect::to("/"), "Member successfully added.")
    }
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    DbConn::get_one(&rocket)
        .await
        .expect("database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("diesel migrations");
        })
        .await;

    rocket
}

pub fn run() -> Rocket<Build> {
    rocket::build()
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Run migration", run_migrations))
        .mount("/", routes![health_check::health_check])
        .mount("/members", routes![new, delete]);
}
