use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

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
        <header>
            <h1 class="text-4xl font-semibold mb-[20px]">
                "~ Adam Rkouni ~"
            </h1>
            <div class="text-xl font-medium flex gap-8">
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
        <footer>
            <div class="inline">
                "Adam Rkouni Mata"
                <br/>
                "Emails: "
                <EmailLink email="hi@arm53.xyz".into()/>
                <EmailLink email="adamrkounimata@gmail.com".into()/>
                <EmailLink email="adam-53-r@protonmail.com".into()/>
                <br/>
                "Socials: "
                <a target="_blank" class="" href="https://github.com/adam-53-r">
                    "github "
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-github-icon lucide-github"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/><path d="M9 18c-4.51 2-5-2-7-2"/></svg>
                    "; "
                </a>
                <a target="_blank" class="" href="https://gitlab.com/adam-53-r">
                    "gitlab "
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-gitlab-icon lucide-gitlab"><path d="m22 13.29-3.33-10a.42.42 0 0 0-.14-.18.38.38 0 0 0-.22-.11.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18l-2.26 6.67H8.32L6.1 3.26a.42.42 0 0 0-.1-.18.38.38 0 0 0-.26-.08.39.39 0 0 0-.23.07.42.42 0 0 0-.14.18L2 13.29a.74.74 0 0 0 .27.83L12 21l9.69-6.88a.71.71 0 0 0 .31-.83Z"/></svg>
                    "; "
                </a>
                <a target="_blank" class="" href="https://www.linkedin.com/in/adam-rkouni/">
                    "linkedin "
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-linkedin-icon lucide-linkedin"><path d="M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"/><rect width="4" height="12" x="2" y="9"/><circle cx="4" cy="4" r="2"/></svg>
                    "; "
                </a>
            </div>
        </footer>
    }
}

#[component]
fn About() -> impl IntoView {
    let content = md_to_html(include_str!("../public/about.md"));

    view! {
        <div inner_html=content class="md_content"/>
    }
}

#[component]
fn Cv() -> impl IntoView {
    let content = md_to_html(include_str!("../public/cv.md"));

    view! {
        <div inner_html=content class="md_content"/>
    }
}

#[component]
fn Blog() -> impl IntoView {
    let content = md_to_html(include_str!("../public/blog.md"));

    view! {
        <div inner_html=content class="md_content"/>
    }
}

#[component]
#[allow(dead_code)]
fn Link(link: String, text: String) -> impl IntoView {
    view! {
        <a target="_blank" class="" href={link}>
            {text}
            " "
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-globe-icon lucide-globe inline"><circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/></svg>
        </a>
        "; "
    }
}

#[component]
fn EmailLink(email: String) -> impl IntoView {
    view! {
        <a class="" href=format!("mailto:{email}")>
            {format!("{email}")}
            " "
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="lucide lucide-mail-icon lucide-mail"><rect width="20" height="16" x="2" y="4" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg>
        </a>
        "; "
    }
}

fn md_to_html(markdown: &str) -> String {
    let mut options = markdown::Options::gfm();
    options.compile.allow_dangerous_html = true;
    markdown::to_html_with_options(markdown, &options)
        .unwrap()
        .replace("<a ", r#"<a target="_blank" "#)
}
