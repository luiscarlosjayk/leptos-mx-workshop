use leptos::*;

#[component]
pub fn ShoppingCartItem() -> impl IntoView {
    view! {
        <div
            class="grid grid-cols-[70px_150px_50px_1fr] gap-2.5 items-center text-center even:bg-stone-600"
        >
            <div>
                <img
                    class="w-full"
                    src="assets/chair-1.png"
                />
            </div>
            <div>"Chair 1"</div>
            <div>"$ 100"</div>
            <div
                class="flex"
            >
                <button class="w-6 h-6 bg-white text-black rounded-full">-</button>
                <span class="w-6 h-6">"1"</span>
                <button class="w-6 h-6 bg-white text-black rounded-full">+</button>
            </div>
        </div>
    }
}
