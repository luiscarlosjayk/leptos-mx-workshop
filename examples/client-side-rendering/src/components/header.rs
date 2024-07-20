use leptos::*;
use leptos_icons::*;
use leptos_router::*;
use leptos_use::use_document;

#[component]
pub fn Header() -> impl IntoView {
    let shopping_cart_button: NodeRef<html::Button> = create_node_ref();

    let open_shopping_cart = move |evt: leptos::ev::MouseEvent| {
        let body = use_document().body().expect("Body element");
        let button = event_target::<web_sys::HtmlButtonElement>(&evt);
        logging::log!("Shopping cart button clicked! {button:?}");

        body.set_attribute("data-active-cart", "true")
            .expect("Have set data-active-cart attribute");
    };

    view! {
        <header>
            <nav class="flex justify-between items-center p-4">
                <div class="flex space-x-8">
                    <A exact=true href="/" class="flex-none flex group">
                        <Icon icon=icondata::TiHomeOutline class="w-6 pr-1 group-hover:hidden inline-block" />
                        <Icon icon=icondata::TiHome class="w-6 pr-1 group-hover:inline-block hidden" />
                        <span>"Home"</span>
                    </A>
                    <A href="products" class="flex-none flex group">
                        <Icon icon=icondata::BiStoreRegular class="w-6 pr-1 group-hover:hidden inline-block" />
                        <Icon icon=icondata::BiStoreSolid class="w-6 pr-1 group-hover:inline-block hidden" />
                        <span>"Store"</span>
                    </A>
                </div>
                <div class="flex justify-end flex-grow">
                    <button
                        class="ml-auto relative group"
                        node_ref=shopping_cart_button
                        on:click=open_shopping_cart
                        title="Open Cart"
                    >
                        <Icon icon=icondata::BiCartAltRegular class="group-hover:hidden inline-block" />
                        <Icon icon=icondata::BiCartAltSolid class="group-hover:inline-block hidden" />
                        <span
                            class="bg-red-600 rounded-full absolute -right-3.5 top-1/2 w-5 h-5 flex justify-center items-center text-white"
                        >"0"</span>
                    </button>
                </div>
            </nav>
        </header>
    }
}
