use flair_general_utils::dao::MySqlDao;
use flair_types::skill::io::{BussinessOutput, BussinessInput};
use log::{debug, info};

use crate::{
    datahelper::{get_session_data, get_user_data, save_session_data, save_user_data},
    models::{session_data::SessionData, user_data::UserData},
};

pub fn action_handler(
    input: BussinessInput,
    session_data: &SessionData,
    user_data: &UserData,
) -> BussinessOutput {
    info!("\naction_handler invoked");
    debug!("\ninput :{:?}", input);
    debug!("\nsession_data :{:?}", session_data);
    debug!("\nuser_data :{:?}", user_data);

    match input.action {
        _ => {
            debug!("\n\nSuccess\n\n")
        }
    }
    BussinessOutput::new()
}

pub async fn bussiness_handler(input: BussinessInput, mut dao: MySqlDao) -> BussinessOutput {
    info!("\nbussiness_handler invoked");
    debug!("\ninput :{:?}", input);
    let session_data = get_session_data(input.clone(), &mut dao).await;
    let user_data = get_user_data(input.clone(), &mut dao);
    let mut _resp = action_handler(input.clone(), &session_data, &user_data);
    save_user_data(input.clone(), &mut _resp, user_data, &mut dao);
    save_session_data(input.clone(), &mut _resp, session_data, &mut dao);
    println!("\n{:?}", _resp);
    _resp
}
