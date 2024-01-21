use yew::prelude::*;

// #[derive(Properties, PartialEq)]
// pub struct FolderIconProps {
// }

#[function_component]
pub fn FolderIcon() -> Html {
    //props: &FolderIconProps
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" height="2rem" width="2rem" viewBox="0 0 50 50"><path d="M7.05 40q-1.2 0-2.1-.925-.9-.925-.9-2.075V11q0-1.15.9-2.075Q5.85 8 7.05 8h14l3 3h17q1.15 0 2.075.925.925.925.925 2.075v23q0 1.15-.925 2.075Q42.2 40 41.05 40Zm0-29v26h34V14H22.8l-3-3H7.05Zm0 0v26Z"/></svg>
    }
}
