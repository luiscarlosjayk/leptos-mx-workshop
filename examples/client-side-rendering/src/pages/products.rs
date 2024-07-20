use leptos::*;
use leptos_router::*;
use products_list::ProductsList;
use single_product::Product;

mod products_list;
mod single_product;

#[component]
pub fn Products() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <Outlet/>
        </div>
    }
}

#[component(transparent)]
pub fn ProductsRoutes() -> impl IntoView {
    view! {
        <Route
            path="products"
            view=|| view! { <Products/> }
        >
            <Route
                path=":id"
                view=|| view! { <Product/> }
            />
            <Route
                path="/"
                view=|| view! { <ProductsList/> }
            />
        </Route>
    }
}
