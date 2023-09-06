use rocket;
use school_calander;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    school_calander::rocket().launch().await?;

    Ok(())
}
