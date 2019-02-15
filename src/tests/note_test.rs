use rocket::local::Client;
use rocket::http::{Status, ContentType};
use serde_json::Value;
use super::helpers::*;
use crate::contexts::cms::NewNote;
use crate::contexts::cms::note::*;
use super::*;

#[test]
fn test_cms() {
    run_test!(|client, conn| {
        let note_new = r#"{ "title": "test", "content": "test", "tag_ids": [1, 2, 3] }"#;
        let mut res =
            client
            .post("/note")
            .header(ContentType::JSON)
            .body(&note_new)
            .dispatch();
        assert_eq!(res.status(), Status::Unauthorized);

        res =
            client
            .put("/note/1")
            .dispatch();
        assert_eq!(res.status(), Status::Unauthorized);

        res =
            client
            .get("/tag")
            .dispatch();
        assert_eq!(res.status(), Status::Unauthorized);

        let tag_new = r#"{ "name": "test" }"#;
        res =
            client
            .post("/tag")
            .header(ContentType::JSON)
            .body(&tag_new)
            .dispatch();
        assert_eq!(res.status(), Status::Unauthorized);
    });
}

#[test]
fn test_cms_by() {
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
        let mut body = res.body().unwrap().into_string().unwrap();
        let user: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&user["name"], "test");

        // ログイン
        let account = r#"{ "name": "", "email": "test@test.test", "password": "testtest" }"#;
        let login_cookie = login(&client, &account).expect("logged in");
        let mut res =
            client
            .get("/")
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::SeeOther);
        assert_eq!(res.headers().get_one("Location"), Some("/editor"));

        // タグ作成
        let mut tag_new = r#"{ "name": "tag1" }"#;
        res =
            client
            .post("/tag")
            .header(ContentType::JSON)
            .body(&tag_new)
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let tag1: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&tag1["name"], "tag1");

        tag_new = r#"{ "name": "tag2" }"#;
        res =
            client
            .post("/tag")
            .header(ContentType::JSON)
            .body(&tag_new)
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let tag2: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&tag2["name"], "tag2");

        // タグ一覧
        res =
            client
            .get("/tag")
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let tags: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&tags[0]["name"], "tag1");
        assert_eq!(&tags[1]["name"], "tag2");

        // ノート作成
        let id_tag1: i32 = tag1["id"].to_string().parse().unwrap();
        let id_tag2: i32 = tag2["id"].to_string().parse().unwrap();
        let mut note_new = NewNote {
            title: "new".to_string(),
            content: "new".to_string(),
            tag_ids: vec![id_tag1, id_tag2]
        };
        res =
            client
            .post("/note")
            .header(ContentType::JSON)
            .body(&serde_json::to_string(&note_new).unwrap())
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let mut note: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&note["title"], &note_new.title);
        assert_eq!(&note["content"], &note_new.content);
        assert_eq!(&note["access"], 0);
        assert_eq!(&note["user_id"], &user["id"]);

        // ノート取得
        res =
            client
            .get(format!("/note/{}", &note["id"]))
            .header(ContentType::JSON)
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        note = serde_json::from_str(&body).unwrap();
        assert_eq!(&note["note"]["title"], &note_new.title);
        assert_eq!(&note["note"]["content"], &note_new.content);
        assert_eq!(&note["note"]["access"], 0);
        assert_eq!(&note["note"]["user_id"], &user["id"]);
        assert_eq!(&note["tags"][0]["name"], "tag1");

        // ノート変更
        note_new = NewNote {
            title: "edited".to_string(),
            content: "edited".to_string(),
            tag_ids: vec![id_tag1, id_tag2]
        };
        res =
            client
            .put(format!("/note/{}", &note["note"]["id"]))
            .header(ContentType::JSON)
            .body(&serde_json::to_string(&note_new).unwrap())
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        note = serde_json::from_str(&body).unwrap();
        assert_eq!(&note["title"], &note_new.title);
        assert_eq!(&note["content"], &note_new.content);
        assert_eq!(&note["access"], 0);
        assert_eq!(&note["user_id"], &user["id"]);

        // ノートをPublicに変更
        res =
            client
            .put(format!("/note/{}/access?mode=2", &note["id"]))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        note = serde_json::from_str(&body).unwrap();
        assert_eq!(&note["access"], 2);

        let username = user["name"].as_str().unwrap();
        // ノート一覧
        res =
            client
            .get(format!("/note/book/{}?page=1&per_page=30", username))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let notes: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&notes[0]["listnote"]["title"], "edited");
        assert_eq!(&notes[0]["listnote"]["access"], 2);

        // ノート一覧
        res =
            client
            .get(format!("/note?page=1&per_page=30"))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let notes: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&notes[0]["listnote"]["title"], "edited");
        assert_eq!(&notes[0]["listnote"]["access"], 2);

        // 文字列で検索
        res =
            client
            .get(format!("/note/book/{}?query=edi&page=1&per_page=30", username))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let notes: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&notes[0]["listnote"]["title"], "edited");

        // タグで検索
        res =
            client
            .get(format!("/note/book/{}?tag={},{}&page=1&per_page=30", username, &tag1["id"], &tag2["id"]))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let notes: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&notes[0]["listnote"]["title"], "edited");

        // 両方で検索
        res =
            client
            .get(format!("/note/book/{}?query=edit&tag={},{}&page=1&per_page=30", username, &tag1["id"], &tag2["id"]))
            .cookie(login_cookie.clone())
            .dispatch();
        assert_eq!(res.status(), Status::Ok);
        body = res.body().unwrap().into_string().unwrap();
        let notes: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(&notes[0]["listnote"]["title"], "edited");
    });
}