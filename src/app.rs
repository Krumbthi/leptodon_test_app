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
                    <li><SideBarLink href="workout" icon=icon::TaskIcon()>Workout</SideBarLink></li>
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
                    <Route path=StaticSegment("/workout") view=Workout/>
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
fn Workout() -> impl IntoView {
    view! {
        <Title text="Workout Tracker" />
        <main class="flex justify-center align-center min-h-full mt-[100px]">
            <div class="w-full max-w-4xl">
                <h1 class="font-bold text-4xl mb-8">Workout Tracker</h1>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
                        <h2 class="text-2xl font-semibold mb-4">Add Exercise</h2>
                        <form class="space-y-4">
                            <div>
                                <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Exercise Name</label>
                                <input type="text" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm dark:bg-gray-700 dark:border-gray-600 dark:text-white" />
                            </div>
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Sets</label>
                                    <input type="number" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm dark:bg-gray-700 dark:border-gray-600 dark:text-white" />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Reps</label>
                                    <input type="number" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm dark:bg-gray-700 dark:border-gray-600 dark:text-white" />
                                </div>
                            </div>
                            <div class="grid grid-cols-2 gap-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Weight (kg)</label>
                                    <input type="number" step="0.5" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm dark:bg-gray-700 dark:border-gray-600 dark:text-white" />
                                </div>
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 dark:text-gray-300">Date</label>
                                    <input type="date" class="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500 sm:text-sm dark:bg-gray-700 dark:border-gray-600 dark:text-white" />
                                </div>
                            </div>
                            <div class="flex justify-end">
                                <Button appearance=ButtonAppearance::Primary>Add Exercise</Button>
                            </div>
                        </form>
                    </div>
                    <div class="bg-white dark:bg-gray-800 p-6 rounded-lg shadow-md">
                        <h2 class="text-2xl font-semibold mb-4">Recent Workouts</h2>
                        <div class="space-y-4">
                            <div class="border-b border-gray-200 dark:border-gray-600 pb-4">
                                <h3 class="font-medium">Bench Press</h3>
                                <p class="text-sm text-gray-600 dark:text-gray-400">3 sets x 10 reps @ 60kg</p>
                                <p class="text-xs text-gray-500 dark:text-gray-500">2024-03-16</p>
                            </div>
                            <div class="border-b border-gray-200 dark:border-gray-600 pb-4">
                                <h3 class="font-medium">Squats</h3>
                                <p class="text-sm text-gray-600 dark:text-gray-400">4 sets x 8 reps @ 80kg</p>
                                <p class="text-xs text-gray-500 dark:text-gray-500">2024-03-15</p>
                            </div>
                            <div>
                                <h3 class="font-medium">Deadlift</h3>
                                <p class="text-sm text-gray-600 dark:text-gray-400">3 sets x 5 reps @ 100kg</p>
                                <p class="text-xs text-gray-500 dark:text-gray-500">2024-03-14</p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </main>
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
