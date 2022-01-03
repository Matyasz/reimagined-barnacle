use wasm_bindgen::JsCast;
use web_sys::{HtmlCollection, HtmlFormElement, HtmlInputElement};

use crate::utilities::traits::FormStyle;

pub struct ValidatedStruct<T> {
    pub data: T,
    pub alerts: Vec<String>,
}

/// Takes an Option<HtmlFormElement> representing data coming from any
/// general form (as long as it implements Default and the FormStyle trait)
/// and validates some basic properties of the data inside, then packages
/// the data found along with any alerts that should be raised.
///
/// # Arguments
///
/// * `form`: Option<HtmlFormElement> - The form element to be processed.
///
/// # Returns
///
/// * ValudatedStruct<T> - The data from the form, packaged into a struct of
///                        type T, alongside any alerts that have been raised
///                        about the contents of the data pushed into a Vec<String>
pub fn validate_form<T>(form: Option<HtmlFormElement>) -> ValidatedStruct<T>
where
    T: FormStyle + Default,
{
    let mut data = T::default();
    let mut alerts: Vec<String> = vec![];

    match form {
        Some(f) => {
            let user_input = f.get_elements_by_class_name("credentials-text");

            for fld in data.fields() {
                let v = process_form_field(&user_input, fld.as_str());

                let msg = validate_form_field(&v, &fld);
                match msg {
                    Some(m) => alerts.push(m),
                    None => {}
                }
                data.set(&fld, v);
            }
        }
        None => {}
    }

    ValidatedStruct { data, alerts }
}

pub fn process_form_field(col: &HtmlCollection, field: &str) -> String {
    let value = col
        .named_item(field)
        .unwrap()
        .dyn_ref::<HtmlInputElement>()
        .unwrap()
        .value();

    // log!(JsValue::from(&value));
    value
}

pub fn validate_form_field(value: &String, field: &str) -> Option<String> {
    if !value.is_empty() {
        return None;
    }

    Some(format!("Please enter a value for {}", field))
}
