import { readable } from "svelte/store";
import { invoke } from "@tauri-apps/api";
import { listen } from '@tauri-apps/api/event';

interface Wallet {
    created_at: number;
    id: string;
    key: string;
}

interface API {
    key: string;
    expiry: number;
}

export interface Settings {
    wallet: Wallet | undefined;
    api: API | undefined;
    password: string | undefined;
    path: string;
    mode: "category" | "list";
}

export const settings = readable<Settings | null>(null, (set) => {
    invoke<Settings>('read_settings').then((storage: Settings) => {
        set(storage)
    })

    const unlisten = listen('update:settings', (event: { payload: Settings }) => {
        set(event.payload)
    })

    return () => {
        unlisten.then((call) => call())
    }
})

export const wallet = readable<Wallet | null>(null, (set) => {
    const unsubscribe = settings.subscribe(value => {
        if (value?.wallet) set(value.wallet);
        else set(null);
    })
    return unsubscribe;
})

export const api = readable<API | null>(null, (set) => {
    const unsubscribe = settings.subscribe(value => {
        if (value?.api) set(value.api);
        else set(null);
    })
    return unsubscribe;
})

export const password = readable<string | null>(null, (set) => {
    const unsubscribe = settings.subscribe(value => {
        if (value?.password) set(value.password);
        else set(null);
    })
    return unsubscribe;
})

export const filepath = readable<string | null>(null, (set) => {
    const unsubscribe = settings.subscribe(value => {
        if (value?.path) set(value.path);
        else set(null);
    })
    return unsubscribe;
})

export const display = readable<"category" | "list" | null>(null, (set) => {
    const unsubscribe = settings.subscribe(value => {
        if (value?.mode) set(value.mode);
        else set(null);
    })
    return unsubscribe;
}) 