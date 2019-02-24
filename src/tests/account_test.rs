use rocket::local::Client;
use rocket::http::{Status, ContentType};
use serde_json::Value;
use super::helpers::*;
use super::*;

#[test]
fn test_new_get_authenticate_delete() {
    run_test!(|client, conn| {
        let account = r#"{ "name": "test", "email": "test@test.test", "password": "testtest" }"#;

        // ユーザー登録
        let mut res =
            client
            .post("/user")
            .header(ContentType::JSON)
            .body(&account)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        let body = res.body().unwrap().into_string().unwrap();
        let user: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&user["name"], "test");

        // 登録したユーザーを取得
        let mut res =
            client
            .get(format!("/user/{}", &user["id"]))
            .header(ContentType::JSON)
            .dispatch();
        let body = res.body().unwrap().into_string().unwrap();
        let user: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(res.status(), Status::Ok);
        assert_eq!(&user["name"], "test");

        // ログイン
        let account = r#"{ "name": "", "email": "test@test.test", "password": "testtest" }"#;
        let login_cookie = login(&client, &account).expect("logged in");
        let mut res =
            client
            .get("/")
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);

        // ユーザー削除
        let mut res =
            client
            .delete(format!("/user/{}", &user["id"]))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::Ok);

        let mut res =
            client
            .get(format!("/user/{}", &user["id"]))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(res.status(), Status::NotFound);
    });
}