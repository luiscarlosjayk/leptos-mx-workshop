use components::{Header, ShoppingCart};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod components;
mod domain;
mod pages;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <div class="container mx-auto group-data-[active-cart=true]/body:-translate-x-10 duration-500">
                <Header/>
                <main>
                    <Routes>
                        <ProductsRoutes/>
                        <Route
                            path=""
                            view=|| view! { <Home/> }
                        />
                        <Route
                            path="*"
                            view=|| view! { <NotFound/> }
                        />
                    </Routes>
                </main>
            </div>
            <ShoppingCart/>
        </Router>
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

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Home"</h2>
        </div>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"404 Not Found"</h2>
        </div>
    }
}

#[component]
fn Products() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <Outlet/>
        </div>
    }
}

#[component]
fn ProductsList() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Products List"</h2>
            <components::ProductsGrid/>
        </div>
    }
}

#[component]
fn Product() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"Product"</h2>
        </div>
    }
}

#[component]
fn NoProduct() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"No Product"</h2>
        </div>
    }
}
