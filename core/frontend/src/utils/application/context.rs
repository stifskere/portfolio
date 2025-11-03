use std::rc::Rc;

use translatable::Language;
use yew::prelude::*;

use crate::utils::language::set_browser_language;


pub type AppContext = Rc<InnerAppContext>;

// TODO: Make an actual app context implementation

#[derive(PartialEq, Debug)]
pub struct InnerAppContext {
    pub language: UseStateHandle<Language>
}

impl InnerAppContext {
    /// This function will set the language preserving
    /// its state in localstorage.
    pub fn set_language(&self, language: Language) {
        set_browser_language(&language);
        self.language.set(language);
    }
}
