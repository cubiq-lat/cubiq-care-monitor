import { localizeHref } from "$lib/paraglide/runtime";

export function getLocaleName(uiLocale: string) {
    // create a DisplayNames instance for language‐type codes
    const languageNames = new Intl.DisplayNames([uiLocale], { type: "language" });
    return languageNames.of(uiLocale) || uiLocale;
}

export function redirectLocale(newLocale: string, path: string = "/") {
    window.location.href = localizeHref(path, { locale: newLocale });
}
