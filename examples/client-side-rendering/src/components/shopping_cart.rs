use super::shopping_cart_item::ShoppingCartItem;
use leptos::*;
use leptos_use::use_document;

#[component]
pub fn ShoppingCart() -> impl IntoView {
    let close_button: NodeRef<html::Button> = create_node_ref();

    let close_shopping_cart = move |evt: leptos::ev::MouseEvent| {
        let body = use_document().body().expect("Body element");
        let button = event_target::<web_sys::HtmlButtonElement>(&evt);
        logging::log!("Close button clicked! {button:?}");

        body.set_attribute("data-active-cart", "false")
            .expect("Have set data-active-cart attribute");
    };
    view! {
        <aside
            class="fixed bg-stone-700 text-slate-200 max-w-full w-96 top-0 -right-96 bottom-0 grid grid-rows-[70px_1fr_70px] group-data-[active-cart=true]/body:right-0 duration-500"
        >
            <h1
                class="text-2xl p-5 m-0 font-light shadow-lg"
            >"Shopping Cart"</h1>
            <div
                class="overflow-auto scrollbar-zero-width"
            >
                <ShoppingCartItem />
                <ShoppingCartItem />
                <ShoppingCartItem />
                <ShoppingCartItem />
                <ShoppingCartItem />
                <ShoppingCartItem />
                <ShoppingCartItem />
            </div>
            <div class="grid grid-cols-2 font-medium">
                <button
                    class="bg-amber-400 hover:bg-orange-50 text-black duration-500 transition-colors uppercase"
                    node_ref=close_button
                    on:click=close_shopping_cart
                >"Close"</button>
                <button
                    class="bg-amber-600 hover:bg-orange-50 text-black duration-500 transition-colors"
                >"Check Out"</button>
            </div>
        </aside>
    }
}
