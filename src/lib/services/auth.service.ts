// src/lib/services/auth.service.ts
import { invoke } from "@tauri-apps/api/core";

export class AuthService {
    static async isLoggedIn(): Promise<boolean> {
        return await invoke<boolean>("is_logged_in");
    }

    static async login(username: string, password: string): Promise<boolean> {
        return await invoke<boolean>("login", { username, password });
    }

    static async logout(): Promise<void> {
        await invoke("logout");
    }
}
