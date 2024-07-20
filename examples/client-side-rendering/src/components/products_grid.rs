use super::product_card::ProductCard;
use leptos::*;

#[component]
pub fn ProductsGrid() -> impl IntoView {
    view! {
        <section
            class="grid grid-cols-4 gap-5"
        >
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
            <ProductCard/>
        </section>
    }
}
