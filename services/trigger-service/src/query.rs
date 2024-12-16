use actix_web::web;
use database;
use database::model::{NewTriggers, Triggers};
use diesel::prelude::*;

pub fn get_triggers(
    db: &web::Data<database::Database>,
) -> Result<Option<Vec<Triggers>>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut connection = db.get_connection();
    let result = triggers.load::<Triggers>(&mut connection);

    match result {
        Ok(result) => Ok(Some(result)),
        Err(err) => {
            eprintln!("Error getting triggers: {:?}", err);
            Err(err)
        }
    }
}

pub fn get_trigger_by_id(
    db: &web::Data<database::Database>,
    trigger_id: i32,
) -> Result<Option<Triggers>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut connection = db.get_connection();

    match triggers
        .find(trigger_id)
        .select(Triggers::as_select())
        .first::<Triggers>(&mut connection)
        .optional()
    {
        Ok(Some(trigger)) => Ok(Some(trigger)),
        Ok(None) => Ok(None),
        Err(err) => {
            eprintln!("Error getting trigger with ID {:?}: {:?}", trigger_id, err);
            Err(err)
        }
    }
}

pub fn create_trigger(
    db: &web::Data<database::Database>,
    _workflow_id: i32,
    _data: &serde_json::Value,
    _status: bool,
    _created_at: Option<chrono::NaiveDateTime>,
) -> Result<Option<Triggers>, diesel::result::Error> {
    use database::schema::triggers;

    let new_trigger = NewTriggers {
        workflow_id: _workflow_id,
        data: _data.clone(),
        status: _status,
        created_at: _created_at,
    };

    let mut connection = db.get_connection();
    diesel::insert_into(triggers::table)
        .values(&new_trigger)
        .execute(&mut connection)?;

    let trigger = triggers::table
        .order(triggers::id.desc())
        .first::<Triggers>(&mut connection)?;

    Ok(Some(trigger))
}

pub fn update_trigger(
    db: &web::Data<database::Database>,
    trigger_id: i32,
    _workflow_id: i32,
    _data: &serde_json::Value,
    _status: bool,
    _created_at: &chrono::NaiveDateTime,
) -> Result<Option<Triggers>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut connection = db.get_connection();

    let trigger = diesel::update(triggers.find(trigger_id))
        .set((
            workflow_id.eq(_workflow_id),
            data.eq(_data),
            status.eq(_status),
            created_at.eq(_created_at),
        ))
        .get_result::<Triggers>(&mut connection)?;

    Ok(Some(trigger))
}

pub fn delete_trigger(
    db: &web::Data<database::Database>,
    trigger_id: i32,
) -> Result<Option<Triggers>, diesel::result::Error> {
    use database::schema::triggers::dsl::*;

    let mut connection = db.get_connection();

    match diesel::delete(triggers.find(trigger_id))
        .get_result::<Triggers>(&mut connection)
        .optional()
    {
        Ok(deleted_trigger) => Ok(deleted_trigger),
        Err(err) => {
            eprintln!(
                "Error deleting trigger with ID {:?}: {:?}",
                trigger_id, err
            );
            Err(err)
        }
    }
}