use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn Error404() -> impl IntoView {
    view! {
        <div class="text-center flex flex-col items-center justify-center w-screen h-screen">
            <h2
                class="p-6 text-4xl flex items-center justify-center gap-3"
            >
                <Icon
                    class="w-14 h-auto"
                    icon=icondata::TbError404
                />
                <span>"Not Found"</span>
            </h2>
            <A
                class="duration-100 shadow-3xl bg-orange-500 hover:bg-orange-700 active:bg-orange-900 text-lg text-orange-50 rounded-3xl border-none px-4 py-3 mt-3 flex justify-center items-center gap-2"
                href="/"
            >
                <span>"Take me Home"</span>
                <Icon class="animate-pulse" icon=icondata::TbClick />
            </A>
        </div>
    }
}
