import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import ViteRsw from "vite-plugin-rsw";
import WindiCSS from "vite-plugin-windicss";

export default defineConfig({
	plugins: [
		WindiCSS(),
		vue(),
		ViteRsw({
			crates: ["hangman"],
		}),
	],
});
