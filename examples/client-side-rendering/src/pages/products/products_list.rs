use crate::components;
use leptos::*;

#[component]
pub fn ProductsList() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Products List"</h2>
            <components::ProductsGrid/>
        </div>
    }
}
