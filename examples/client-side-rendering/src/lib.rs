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
            <Routes>
                <Route
                    path="/"
                    view=|| view! {
                        <div class="container mx-auto group-data-[active-cart=true]/body:-translate-x-10 duration-500">
                            <Header/>
                            <main>
                                <Outlet/>
                            </main>
                        </div>
                        <ShoppingCart/>
                    }
                >
                    <pages::products::ProductsRoutes/>
                    <Route
                        path=""
                        view=|| view! { <pages::home::Home/> }
                    />
                </Route>
                <Route
                    path="*"
                    view=|| view! { <pages::error_404::Error404/> }
                />
            </Routes>
        </Router>
    }
}
