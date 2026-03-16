// Leptodon-starter
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use leptos::prelude::*;
use leptodon::button::{Button, ButtonAppearance};
use leptodon::darkmode::ThemeSelector;
use leptodon::icon;
use leptodon::navbar::NavbarEndChildren;
use leptodon::navbar::NavbarEntries;
use leptodon::navbar::SideBarLink;
use leptodon::navbar::SideNavbar;
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::components::Outlet;
use leptos_router::components::ParentRoute;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

const NAME: &str = "Leptodon";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="min-h-full">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                // Metadata injection is not allowed here, only use them in components down the chain
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="min-h-full">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn RouteShell() -> impl IntoView {
    view! {
        <main>
            <SideNavbar>
                <NavbarEntries slot:entries>
                    <li><SideBarLink href="home" icon=icon::HomeIcon()>Home</SideBarLink></li>
                    <li><SideBarLink href="test" icon=icon::InfoIcon()>Info</SideBarLink></li>
                </NavbarEntries>
                <NavbarEndChildren slot:end>
                    <ThemeSelector />
                </NavbarEndChildren>

                <div class="mt-[100px]">
                    <Outlet />
                </div>
            </SideNavbar>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
    view! {
        <Stylesheet href="/pkg/starter.css"/>

        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=StaticSegment("/") view=RouteShell>
                    <Route path=StaticSegment("/home") view=Home/>
                    <Route path=StaticSegment("/test") view=Test/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Welcome" />
        <p>Hello World!</p>
    }
}

#[component]
fn Test() -> impl IntoView {
    view! {
        <Title text=NAME />
        <main class="flex justify-center align-center min-h-full mt-[100px]">
            <div>
                <h1 class="font-bold text-4xl">{NAME}</h1>
                <h2 class="text-xl">Your Leptos UI toolkit</h2>
                <Button appearance=ButtonAppearance::Primary>Docs</Button>
                <Button appearance=ButtonAppearance::Primary>Demo</Button>
                <Button>Crate</Button>
                <Button>Github</Button>
            </div>
        </main>
    }
}