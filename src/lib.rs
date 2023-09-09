#[macro_use]
extern crate rocket;

pub mod calendar;
pub mod models;

#[launch]
pub fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![
            get_calander
        ]
    )
}

#[get("/cal.ics")]
pub async fn get_calander() -> String {
    let cal = calendar::get_calander();

    format!("{}", cal)
}