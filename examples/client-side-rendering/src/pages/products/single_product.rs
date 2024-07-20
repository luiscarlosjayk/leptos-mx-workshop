use crate::components::ProductsGrid;
use leptos::*;
use leptos_router::A;

#[component]
pub fn Product() -> impl IntoView {
    view! {
        <div class="my-0 mx-auto max-w-3xl text-center">
            <h2 class="p-6 text-4xl">"PRODUCT DETAIL"</h2>

            <section
                class="grid grid-cols-2 gap-14 text-left"
            >
                <A
                    class="relative"
                    href="/products/1"
                >
                    <span
                        class="w-72 h-72 bg-zinc-200 inline-block absolute z-[-1] rounded-[190px_100px_170px_180px] top-12 left-[calc(50%-150px)]"
                    ></span>
                    <img
                        class="drop-shadow-4xl w-full"
                        src="/assets/chair-1.png"
                    />
                </A>
                <div>
                    <h3
                        class="font-medium text-3xl pt-10 mb-2.5"
                    >"Product Name"</h3>
                    <div
                        class="text-sm tracking-widest text-xl font-bold mb-5"
                    >"$ 200"</div>
                    <div
                        class="flex justify-start items-center gap-3">
                        <button
                            class="duration-75 bg-slate-200 hover:bg-slate-300 active:bg-slate-400 text-xl text-slate-900 rounded-3xl border-none px-3 py-2.5 mt-2.5"
                        >"Check Out"</button>
                        <button
                            class="duration-75 bg-slate-900 hover:bg-orange-500 active:bg-orange-700 text-xl text-orange-50 rounded-3xl border-none px-3 py-2.5 mt-2.5 drop-shadow-[0_10px_20px_#2F2F2F77]"
                        >
                            <span>"Add to Cart"</span>
                        </button>
                    </div>
                    <div
                        class="text-md font-light mt-5">
                        "Slim-fitting style, contrast raglan long sleeve, three-button henley placket, light weight & soft fabric for breathable and comfortable wearing. And Solid stitched shirts with round neck made for durability and a great fit for casual fashion wear and diehard baseball fans. The Henley style round neckline includes a three-button placket."
                    </div>
                </div>
            </section>

            <section>
                <div class="p-6 text-4xl">"Similar Products"</div>
                <ProductsGrid />
            </section>
        </div>
    }
}
