// Tauri API type declarations for injected globals
interface Window {
    __TAURI__: {
        core: {
            invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
        };
        event: {
            listen: (event: string, handler: (event: { payload: unknown }) => void) => Promise<() => void>;
            emit: (event: string, payload?: unknown) => Promise<void>;
        };
        notification: {
            sendNotification: (options: { title: string; body?: string }) => void;
        };
    };
}