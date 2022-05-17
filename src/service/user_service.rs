use crate::dao::user_dao::UserDao;

use crate::domain::dto::user::UserRegisterDto;
use crate::domain::entity::user::User;

use mybatis::snowflake;
use mybatis::DateTimeNative;

pub async fn save_user(user_dto: &UserRegisterDto) {
    log::info!("userdto: {:?}", user_dto);

    let snowflake_id: Option<i64> = Some(snowflake::new_snowflake_id());

    let user = User {
        id: snowflake_id,
        user_name: user_dto.user_name.clone(),
        user_password: user_dto.user_password.clone(),
        name: user_dto.name.clone(),
        phone: user_dto.phone.clone(),
        sex: user_dto.sex,
        create_time: DateTimeNative::now().into(),
        status: Some(0),
    };

    UserDao::user_register(&user).await;
}

#[test]
fn test_now() {
    let dt = DateTimeNative::now();
    println!("{}", dt);
    let s = rbson::to_bson(&dt).unwrap();
    let dt_new: DateTimeNative = rbson::from_bson(s).unwrap();
    println!("{},{}", dt.timestamp_millis(), dt_new.timestamp_millis());
    assert_eq!(dt, dt_new);
}