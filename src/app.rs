use leptos::{prelude::*, tachys::view};
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

include!(concat!(env!("OUT_DIR"), "/hello.rs"));

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/style/output.css"/>
        // <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Header/>
        <Router>
            <Routes fallback=|| "Page not found.">
                <Route path=StaticSegment("") view=About/>
                <Route path=StaticSegment("cv") view=Cv/>
                <Route path=StaticSegment("blog") view=Blog/>
            </Routes>
        </Router>
        <Footer/>
    }
}

#[component]
fn Header() -> impl IntoView {
    view! {
        <header class="px-20 pt-[3rem] mx-auto w-1/2">
            <h1 class="text-4xl font-medium mb-[20px]">
                "~ Adam Rkouni ~"
            </h1>
            <div class="mb-5 text-2xl font-medium flex gap-8">
                <a class="hover:underline" href="/">
                    "About me"
                </a>
                <a class="hover:underline" href="/cv">
                    "CV"
                </a>
                <a class="hover:underline" href="/blog">
                    "Blog"
                </a>

            </div>
        </header>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <footer class="px-20 pb-[4rem] mx-auto w-1/2">
            <div class="">
                "Adam Rkouni Mata"
                <br/>
                "Emails: "
                <EmailLink email="hi@arm53.xyz".into()/>
                <EmailLink email="adam-53-r@protonmail.com".into()/>
                <EmailLink email="adam-53-r@pm.me".into()/>
                <br/>
                "Socials: "
                <Link link="https://github.com/adam-53-r".into() text="github".into()/>
                <Link link="https://gitlab.com/adam-53-r".into() text="gitlab".into()/>
                <Link link="https://www.linkedin.com/in/adam-rkouni/".into() text="linkedin".into()/>
            </div>
        </footer>
    }
}

#[component]
fn About() -> impl IntoView {

    let mut options = markdown::Options::default();
    options.compile.allow_dangerous_html = true;
    let content = markdown::to_html_with_options(include_str!("../test.md"), &options).unwrap();

    view! {        
        <main class="px-20 pt-[1rem] pb-[4rem] mx-auto w-1/2">

            <h2 class="text-3xl font-medium mb-[20px]">
                "About me"
            </h2>
            <div inner_html=content >

                // "Lorem ipsum dolor sit amet consectetur adipisicing elit. Eos architecto, in nemo eaque sunt iure, officiis dolorem rem ipsam velit cum laudantium magni suscipit dignissimos quod debitis. Expedita, consequuntur iste?"

                

            </div>
        </main>
    }
}

#[component]
fn Cv() -> impl IntoView {

    view! {}
}

#[component]
fn Blog() -> impl IntoView {

    view! {}
}

#[component]
fn Link(link: String, text: String) -> impl IntoView {
    view! {
        <a class="text-yellow-300 hover:text-yellow-500" href={link}>{text}</a>"; "
    }
}

#[component]
fn EmailLink(email: String) -> impl IntoView {
    view! {
        <div class="flex flex-row">
            <a class="text-yellow-300 hover:text-yellow-500" href=format!("mailto:{email}")>{format!("{email}")}</a>
            // <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-mail-icon lucide-mail"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
            "; "
        </div>
    }
}