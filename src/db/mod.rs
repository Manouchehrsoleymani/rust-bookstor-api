/*
*  Bookstor API Project
*
* @author Manouchehr Soleymani
* @link https://github.com/Manouchehrsoleymani/rust-bookstor-api
*/

use sea_orm::*;
use crate::AppConfig;

// pub(super) async fn connect(config: &AppConfig) -> Result<DatabaseConnection, DbErr>{
//     let mut opts = ConnectOptions::new( format!(
//         "mysql://{}:{}@{}:{}/{}",
//         config.db_username,config.db_password,config.db_host,config.db_port,config.db_name,
//     ));
//     opts.sqlx_logging(false);
//     Database::connect(opts).await
// }

pub(super) async fn setup_db(config: &AppConfig) -> Result<DatabaseConnection, DbErr> {
    let mut opts = ConnectOptions::new( format!(
        "mysql://{}:{}@{}:{}",
        config.db_username,config.db_password,config.db_host,config.db_port,
    ));
    opts.sqlx_logging(false);
    

    let db = Database::connect(opts).await?;

    let db = match db.get_database_backend() {
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", config.db_name),
            ))
            .await?;

            Database::connect(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username,config.db_password,config.db_host,config.db_port,config.db_name,
    )).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", config.db_name),
            ))
            .await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", config.db_name),
            ))
            .await?;

            Database::connect(format!(
        "mysql://{}:{}@{}:{}/{}",
        config.db_username,config.db_password,config.db_host,config.db_port,config.db_name,
    )).await?
        }
        DbBackend::Sqlite => db,
    };

    Ok(db)
}
