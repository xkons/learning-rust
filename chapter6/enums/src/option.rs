use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct FormData {
    mandatory_field: String,
    optional_field: Option<String>
}

fn sanitize_form_data(mandatory_field: &String, optional_field: &Option<String>) -> FormData {
    let sanitized_mandatory_field = mandatory_field.trim().to_owned();
    let sanitized_optional_field: Option<String> = match optional_field {
        Some(field_value) => {
            Some(field_value.trim().to_owned())
        },
        None => None
    };
    return FormData {
        mandatory_field: sanitized_mandatory_field,
        optional_field: sanitized_optional_field
    }
}

pub fn main() {
    let mandatory_field = String::from("     Foo");
    let optional_field = Some(String::from("   HI there!"));
    let form_data = sanitize_form_data(&mandatory_field, &optional_field);
    // Serialize it to a JSON string.
    // The None value of the Rust Option enum maps to null in JSON
    let json_string = serde_json::to_string(&form_data);
    match json_string {
        Ok(string) => println!("{}", string), // Print, write to a file, or send to an HTTP server.
        Err(err) => println!("Error occured: {}", err)
    }
}