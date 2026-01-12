<!-- src/routes/login/+page.svelte -->
<script lang="ts">
    import { AuthService } from "$lib/services/auth.service";
    import { goto } from "$app/navigation";
    import { localizeHref } from "$lib/paraglide/runtime";
    import { m } from "$lib/paraglide/messages";
    import { enhance } from "$app/forms";
    import { Eye, EyeOff } from "@lucide/svelte";
    import LanguageButton from "$lib/components/buttons/LanguageButton.svelte";
    import HeaderLoginButtons from "$lib/components/header/HeaderLoginButtons.svelte";

    let username = $state("");
    let password = $state("");
    let showPassword = $state(false);
    let error = $state("");
    let loading = $state(false);

    async function handleLogin() {
        error = "";
        loading = true;

        try {
            const success = await AuthService.login(username, password);
            if (success) {
                await goto("/");
            }
        } catch (e) {
            error = e as string;
        } finally {
            loading = false;
        }
    }
</script>

<!-- <HeaderDefined headerButtons="login" /> -->

<!-- Header -->
<div class="flex items-center justify-between px-6 py-4 border-b border-zinc-800 shrink-0">
    <!-- Right -->
    <div class="flex items-center justify-center gap-4">
        <img src="/images/logos/cubiq-logo-white.svg" alt="cubiq-logo" class="h-8 relative -top-px" />
        <span class="text-zinc-500">></span>
        <span class="text-zinc-400 text-lg">{m.login()}</span>
        <!-- <span class="text-zinc-400 text-lg">{m.example_message({ username: "Pepe" })}</span> -->
    </div>

    <!-- Left -->
    <div class="flex items-center gap-3">
        <!-- <ConfigButton /> -->
        <HeaderLoginButtons />

        <LanguageButton />
    </div>
</div>

<div class="max-w-fw border-x-base-200 relative container my-0 text-center md:mx-auto md:border-x">
    <div class="mx-(--cubiq-app-margin) flex h-[calc(100vh-90px)] flex-1 items-center justify-center">
        <div class="relative -top-8 w-100">
            <h2 class="text-base-content font-sans text-3xl font-black">{m.login()}</h2>
            <div class="mx-auto">
                <form
                    action="?/email"
                    method="POST"
                    onsubmit={handleLogin}
                    use:enhance={() => {
                        // isSubmitting = true;
                        // toggleLoader();
                        // return async ({ update }) => {
                        //     toggleLoader();
                        //     isSubmitting = false;
                        //     update();
                        // };
                    }}
                >
                    <!-- Email input -->
                    <fieldset class="mt-5 mb-3">
                        <label for="email" class="mb-2 ml-2 block text-left text-sm font-medium">Email</label>
                        <input
                            type="email"
                            name="email"
                            value={username ?? ""}
                            id="email"
                            required={true}
                            disabled={loading}
                            placeholder={m.loginYourEmail()}
                            class="focus:ring-primary w-full rounded-md border border-white bg-transparent px-4 py-2 text-white focus:ring-2 focus:outline-none"
                        />
                    </fieldset>

                    <fieldset class="mb-4">
                        <label for="password" class="mb-2 ml-2 block text-left text-sm font-medium">
                            {m.loginPassword()}
                        </label>
                        <div class="relative mt-1">
                            <input
                                id="password"
                                name="password"
                                type={password ? "text" : "password"}
                                bind:value={password}
                                disabled={loading}
                                required
                                class="focus:ring-primary w-full rounded-md border border-white bg-transparent px-4 py-2 text-white focus:ring-2 focus:outline-none"
                                placeholder={m.loginYourPassword()}
                            />
                            <button
                                type="button"
                                class="absolute top-1/2 right-3 -translate-y-1/2 transform text-gray-500 hover:text-gray-700 focus:outline-none"
                                onclick={() => (showPassword = !showPassword)}
                            >
                                {#if showPassword}
                                    <EyeOff class="h-5" />
                                {:else}
                                    <Eye class="h-5" />
                                {/if}
                            </button>
                        </div>
                    </fieldset>

                    <!-- Continue button -->
                    <button
                        type="submit"
                        disabled={loading}
                        class="bg-primary mb-5 w-full rounded-md py-2 font-semibold text-white transition hover:opacity-50"
                    >
                        {loading ? m.loggingIn() : m.continue()}
                    </button>
                    <!-- 
                    {#if form?.missing === "email"}
                        <p class="fw-login-error">{m.loginEmailRequired()}</p>
                    {/if}
                    {#if form?.missing === "password"}
                        <p class="fw-login-error">{m.loginPasswordRequired()}</p>
                    {/if}
                    {#if form?.incorrect}
                        <p class="fw-login-error">{m.loginInvalidCredentials()}</p>
                    {/if} -->

                    <!-- Divider -->
                    <div class="mb-5 flex items-center gap-2 text-sm text-gray-400">
                        <div class="h-px grow bg-gray-600"></div>
                        <span class="uppercase">{m.or()}</span>
                        <div class="h-px grow bg-gray-600"></div>
                    </div>

                    <!-- Google button -->
                    <!-- <button
						class="mb-3 flex w-full items-center justify-center gap-2 rounded-md bg-white py-2 font-medium text-black shadow-sm transition hover:shadow-md"
					>
						<img
							src="https://www.svgrepo.com/show/475656/google-color.svg"
							alt="Google"
							class="h-5 w-5"
						/>
						{m.loginContinueWithGoogle()}
					</button> -->

                    <!-- Nostr button -->
                    <!-- <button
                        onclick={() => {
                            alert(`${m.soon()}`);
                        }}
                        type="button"
                        class="mb-3 flex hidden! w-full items-center justify-center gap-2 rounded-md bg-white py-2 font-medium text-black shadow-sm transition hover:shadow-md"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="18" height="18">
                            <path
                                fill="black"
                                d="M412.5 22.17c-.6 0-1.2.1-1.7.1c-9 .72-15.4 5.89-16.2 8.96c-2.1 7.02-3.6 16.36-2.2 22.82s3.4 10.34 14.2 12.22c16.6 2.88 35.4-.64 51.8-6.43c13-4.61 24.2-10.62 31.3-15.14c-.4-.67-.8-1.27-1.4-1.78c-1.5-1.26-4.3-2.53-8.6-3.31c-8.7-1.57-22.2-.88-36-1.57l-4.3-.22l-2.5-3.5c-6.2-8.57-14.6-11.93-22.7-12.14h-1.7zm-14 60.61c-3.9 10.17-4.5 20.22-2.8 29.52c2.2 12.2 9.5 22.1 13.6 32.9c14 36.6.8 45.4-20.8 51.1c22.3 20 33.3 44.4 35 68.3c30-45.7 35.3-86.2 1.3-128.6c-6.7-8.3-9.9-18.2-11.4-26.8c-1.5-7.9-.8-15.35 3.7-23.91c-4.5-.11-9-.51-13.5-1.29c-1.8-.31-3.4-.72-5.1-1.22M108.4 126.9c-29.04-.2-53.3 25.3-56.66 60c10.56-10.7 25.02-17.7 46.11-17.2c-20.2 13.7-33.69 29.2-44.34 45.3c1.77 8.2 4.64 16.5 8.8 24.9c38.09-52.5 60.99-29.6 72.69.1c10.8-20 27.4-36 47-48.1c-21.4-46.4-49.2-64.8-73.6-65m180.2 55c-3.4 0-6.9 0-10.4.2c-12.4.5-25 2.2-37.3 5.1c-46.3 11-86.3 38-97.9 81.1c22.2 7.6 40.7 16.4 56.7 25.4c31.6 4.7 63.8 3.2 91.4-2.5c38.5-8 67-25.4 73.3-39.7l16.4 7.2c-11 25.4-44.7 41.6-86 50.1c-19 4-39.9 6-61.3 5.7c4.3 2.8 8.4 5.5 12.4 8.1c27.1 17.6 48.4 29.7 82.6 28c35.6-2.9 62.6-25.9 72.9-54.6c10.2-28.7 4.4-62.5-28.7-89.5c-19.2-15.8-50.6-24.4-84.1-24.6m-92.5 130.4c-4.4 8.8-8.1 18-10.6 26.2c-18.6 8.8-25 12.1-34.8 33.2c-35.5 15.3-50.4 38.2-61.34 71.5c-22.48 6.1-40.85 5-63.06-9.9c-12.14 16.1 6.05 30.4 22.64 36.9c21.16 8.3 50.31 2.8 55.46-7.2c16.5-32.5 31.1-54.7 61.7-77c13.1-1.4 22.4-2.4 34.3-18c21.6 2.2 39.3-8 55-18.1c7.8 17.9 23.5 41.8 20.7 58.9c-11.8 9.5-8 21.3-8.3 24.1c6 17.9 66.2-5.8 108 18c22.4 12.8 27.9 44.9 69.4 37.9c6.8-1.1 5.3-7.2.6-8.9c-37.4-13.7-27.6-21-44.8-48.1c-32-.8-59.3-5.2-95.4-16.1c-10.4-3 3.2-24.8 13.3-47c-33.4-.9-57.8-14.7-82.9-31c-12.4-8.1-25.2-16.8-39.9-25.4"
                            />
                        </svg>
                        Continuar com Nostr (NIP-07)
                    </button> -->
                    <!-- Google login -->
                    <!-- <button
                        onclick={() => {
                            // createUser();
                            alert(`${m.soon()}`);
                        }}
                        type="button"
                        class="mb-3 flex w-full items-center justify-center gap-2 rounded-md bg-white py-2 font-medium text-black shadow-sm transition hover:shadow-md"
                    >
                        Continuar com Google
                    </button> -->

                    <!-- Register prompt -->
                    <p class="text-center text-sm">
                        {m.loginDontHaveAccount()}
                        <a href={localizeHref("/soon")} class="text-primary ml-1 font-bold uppercase hover:underline">
                            {m.loginStartNow()}
                        </a>
                    </p>
                </form>
            </div>
        </div>
    </div>

    <!-- Decoration 1 -->
    <div class="absolute -top-[5%] right-[20%] z-10 scale-40 opacity-60 md:bottom-[10%]">
        <img src="/images/bgs/cross.svg" alt="cross" />
    </div>
    <!-- Decoration 2 -->
    <div class="absolute -top-[5%] -left-[10%] z-10 scale-40 opacity-80 md:bottom-[10%]">
        <img src="/images/bgs/cross.svg" alt="cross" />
    </div>
</div>

<!-- 
<div class="login-container">
    <h1>Cubiq Care Monitor</h1>
    <h2>Login</h2>

    <form on:submit|preventDefault={handleLogin}>
        <div class="form-group">
            <label for="username">Username</label>
            <input id="username" type="text" bind:value={username} placeholder="Enter username" disabled={loading} />
        </div>

        <div class="form-group">
            <label for="password">Password</label>
            <input
                id="password"
                type="password"
                bind:value={password}
                placeholder="Enter password"
                disabled={loading}
            />
        </div>

        <button type="submit" disabled={loading}>
            {loading ? "Logging in..." : "Login"}
        </button>

        {#if error}
            <p class="error">{error}</p>
        {/if}
    </form>
</div>

<style>
    .login-container {
        max-width: 400px;
        margin: 100px auto;
        padding: 40px;
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
    }

    h1 {
        text-align: center;
        margin-bottom: 10px;
        font-size: 24px;
    }

    h2 {
        text-align: center;
        margin-bottom: 30px;
        font-size: 18px;
        color: #666;
    }

    .form-group {
        margin-bottom: 20px;
    }

    label {
        display: block;
        margin-bottom: 5px;
        font-weight: 500;
    }

    input {
        width: 100%;
        padding: 10px;
        border: 1px solid #ddd;
        border-radius: 4px;
        font-size: 14px;
        box-sizing: border-box;
    }

    input:disabled {
        background-color: #f5f5f5;
        cursor: not-allowed;
    }

    button {
        width: 100%;
        padding: 12px;
        margin-top: 10px;
        background-color: #007bff;
        color: white;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        cursor: pointer;
    }

    button:hover:not(:disabled) {
        background-color: #0056b3;
    }

    button:disabled {
        background-color: #ccc;
        cursor: not-allowed;
    }

    .error {
        color: #dc3545;
        margin-top: 15px;
        text-align: center;
    }
</style> -->
