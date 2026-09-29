use crate::db::model::Car;

pub async fn init_db() -> anyhow::Result<toasty::Db> {
    let db = toasty::Db::builder()
        .models(toasty::models!(Car))
        .connect("turso:./app.db")
        .await?;

    if db.schema().db.tables.is_empty() {
        db.push_schema().await?;
    }

    Ok(db)
}
