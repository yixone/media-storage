import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { readdirSync } from "node:fs";
import { resolve } from "node:path";

export default defineConfig({
    plugins: [react(), tailwindcss()],
    resolve: {
        alias: {
            ...readdirSync(resolve(__dirname, "lib")).reduce(
                (p, f) => ({
                    ...p,
                    [`@lib/${f}`]: resolve(__dirname, "lib", f),
                }),
                {}
            ),
        },
    },
});
