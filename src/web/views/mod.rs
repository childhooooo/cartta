pub mod user_view;
pub mod note_view;

use std::env;
use tera::{GlobalFn, Value, to_value, Result};

pub fn id_ga() -> GlobalFn {

    Box::new(|_| -> Result<Value>  {
        let id = match env::var("ID_GA") {
            Ok(id) => Ok(id),
            Err(_) => Err("$ID_GA is not defined".to_string())
        }?;
        let script = format!("
                <script async src=\"https://www.googletagmanager.com/gtag/js?id={0}\"></script>
                <script>
                    window.dataLayer = window.dataLayer || [];
                    function gtag(){{dataLayer.push(arguments);}}
                    gtag('js', new Date());

                    gtag('config', '{0}');
                </script>
            ", id);
        Ok(to_value(script).unwrap())
    })

}