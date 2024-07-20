use leptos::*;

#[component]
pub fn ProductCard() -> impl IntoView {
    view! {
        <div
            class="bg-orange-50 p-5 rounded-3xl"
        >
            <img
                class="drop-shadow-4xl w-11/12"
                src="assets/chair-1.png"
            />
            <h2
                class="font-medium text-lg"
            >"Chair"</h2>
            <div
                class="text-sm tracking-widest"
            >"$ 200"</div>
            <button
                class="duration-75 bg-slate-900 hover:bg-orange-500 active:bg-orange-700 text-xs text-orange-50 rounded-3xl border-none px-3 py-2.5 mt-2.5"
            >
                <span>"Add to Cart"</span>
            </button>
        </div>
    }
}
