import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
    plugins: [react()],
    server: {
        port: 5173, // Tauri の設定に一致させる
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    root: '.', // 必要に応じて調整
    build: {
        outDir: '../dist', // 必要に応じて調整
    },
});

