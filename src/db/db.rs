use crate::db::model::Car;

pub async fn init_db() -> anyhow::Result<toasty::Db> {
    let db = toasty::Db::builder()
        .models(toasty::models!(Car))
        .connect("turso:./app.db")
        .await?;

    db.push_schema().await?;

    Ok(db)
}
