// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps

import { AuthService } from "$lib/services/auth.service";
import { redirect } from "@sveltejs/kit";

// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export async function load({ url }) {
    const isLoggedIn = await AuthService.isLoggedIn();

    // If not logged in and not on login page, redirect to login
    if (!isLoggedIn && url.pathname !== "/login") {
        throw redirect(302, "/login");
    }

    // If logged in and on login page, redirect to home
    if (isLoggedIn && url.pathname === "/login") {
        throw redirect(302, "/");
    }

    return {
        isLoggedIn,
    };
}
