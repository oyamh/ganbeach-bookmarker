use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/src/js/easy-mde.js")]
extern "C" {
    pub type EasyMDEWrapper;

    #[wasm_bindgen(constructor)]
    fn new(base_element_id: String) -> EasyMDEWrapper;

    #[wasm_bindgen(method)] //, setter
    fn set_value(this: &EasyMDEWrapper, content: String);
    #[wasm_bindgen(method)] // , getter
    fn get_value(this: &EasyMDEWrapper) -> String;
    #[wasm_bindgen(method)]
    fn focus(this: &EasyMDEWrapper);
}

#[hook]
pub fn use_annotation() -> AnnotationRef {
    log::debug!("use_annotation");
    let annotation_ref = AnnotationRef {
        easy_mde_ref: use_state(|| None),
    };

    {
        let annotation_ref = annotation_ref.clone();
        use_effect_with_deps(
            move |_| {
                annotation_ref.set(Some(EasyMDEWrapper::new("annotation-editor".to_string())));
                move || annotation_ref.set(None)
            },
            (),
        );
    }
    annotation_ref
}

pub struct AnnotationRef {
    easy_mde_ref: UseStateHandle<Option<EasyMDEWrapper>>,
}

impl AnnotationRef {
    pub fn set(&self, easy_mde: Option<EasyMDEWrapper>) {
        self.easy_mde_ref.set(easy_mde)
    }

    pub fn value(&self) -> Option<String> {
        self.easy_mde_ref.as_ref().map(|editor| editor.get_value())
    }
}

impl Clone for AnnotationRef {
    fn clone(&self) -> Self {
        Self {
            easy_mde_ref: self.easy_mde_ref.clone(),
        }
    }
}
