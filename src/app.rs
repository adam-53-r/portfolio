use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/style/output.css"/>
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        // <Router>
        //     <Routes fallback=|| "Page not found.">
        //         <Route path=StaticSegment("") view=Home/>
        //     </Routes>
        // </Router>
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {

    view! {
        <main class="p-20 mx-auto w-1/2 h-screen text-white break-words font-('Fira Sans', sans-serif) text-xl">
            <h1 class="text-5xl font-medium mb-[20px]">
                "Adam Rkouni"
            </h1>

            <h2 class="text-3xl font-medium mb-[20px]">
                "About me"
            </h2>
            Lorem ipsum dolor sit amet consectetur adipisicing elit. Facere sunt aliquid eaque quidem, amet temporibus accusamus? Culpa minus, quis temporibus optio, quia eveniet quam asperiores labore iure, non molestiae suscipit?
            <br/>
            Lorem ipsum dolor sit amet, consectetur adipisicing elit. Aspernatur quas tempore voluptatibus dolorem ipsa amet, numquam harum recusandae, saepe, nam molestias nesciunt eaque eos deleniti sapiente porro? Veritatis, vel debitis!
            
        </main>
    }
}