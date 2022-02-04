<template>
	<main class="h-screen">
		<div class="pt-8 layout">
			<div class="flex flex-col">
				<div class="text-center font-bold text-7xl text-light-50 mb-18">
					<h1>Hangman</h1>
				</div>
				<div class="flex justify-center mb-8">
					<p class="py-1 px-4 text-purple-300 rounded-lg border-2 border-purple-300 text-3xl select-none shadow-md shadow-true-gray-700"><em class="font-bold not-italic">{{ lives }}</em> lives left</p>
				</div>
				<div class="flex justify-center" v-if="guessStatus">
					<h2 class="py-1 px-4 rounded-lg bg-purple-600 text-light-50 text-center font-bold text-4xl tracking-widest select-none shadow-md shadow-true-gray-700">{{ guessStatus.toUpperCase() }}</h2>
				</div>
				<div class="flex flex-col justify-center content-center items-center my-16 min-h-16">
					<p class="text-center font-bold text-2xl text-light-900 text">{{ msg }}</p>
					<p v-if="word" class="text-center mt-2 text-3xl text-light-900">The word was <em class="not-italic font-bold text-purple-500">{{ word }}</em></p>
				</div>
			</div>
			<div class="flex flex-col justify-center mt-auto mb-8">
				<ul
					v-for="line in keyboardQwerty"
					:key="line"
					class="inline-flex flex-row justify-center not-last:pb-1"
				>
					<li v-for="key in line" :key="key.letter" class="not-last:pr-1">
						<Char @click="guessLetter(key.letter)" :class="[key.color, key.hover]">{{ key.letter.toUpperCase() }}</Char>
					</li>
				</ul>
				<div class="flex flex-col justify-center content-center items-center my-4 text">
					<p class="text-center leading-relaxed text-xs text-true-gray-500">Made with ❤️‍🔥 by Ekkaia</p>
					<p class="text-center leading-relaxed text-xs text-true-gray-500">Written with Rust 🦀, compiled in WebAssembly, embedded in Vue 3 and powered by Vite ⚡</p>
				</div>
			</div>
		</div>
	</main>
</template>

<script>
import Card from "./components/Card.vue";
import Button from "./components/Button.vue";
import Char from "./components/Char.vue";
import init, { Hangman } from "hangman";

export default {
	name: "app",
	components: {
		Card,
		Button,
		Char,
	},
	data() {
		return {
			lives: null,
			game: null,
			word: '',
			msg: '',
			guessStatus: null,
			keyboardQwerty: [
				[
					{
						letter: "q",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "w",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "e",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "r",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "t",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "y",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "u",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "i",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "o",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "p",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					}
				], [
					{
						letter: "a",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "s",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "d",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "f",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "g",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "h",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "j",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "k",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "l",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
				], [
					{
						letter: "z",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "x",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "c",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "v",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "b",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "n",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					},
					{
						letter: "m",
						color: "bg-true-gray-700",
						hover: "hover:bg-true-gray-600"
					}
				],
			],
		};
	},
	async mounted() {
		await init(await fetch("../../hangman/pkg/hangman_bg.wasm"));
		this.game = new Hangman();
		this.guessStatus = await this.game.get_guess_status();
		this.lives = this.game.get_lives();
	},
	watch: {
		lives(val) {
			if (val <= 0) {
				this.word = this.game.get_word();
			}
		}
	},
	methods: {
		guessLetter(char) {
			if (this.game.is_alive() && !this.game.is_already_guessed(char)) {
				const lives = this.lives;
				this.msg = this.game.validate_user_guess(char);
				this.guessStatus = this.game.get_guess_status();
				this.lives = this.game.get_lives();
				if (this.lives !== lives) {
					this.searchForKey(char, "bg-red-700", "hover:bg-red-600");
				} else {
					this.searchForKey(char, "bg-emerald-700", "hover:bg-emerald-600");
				}
			}
		},
		searchForKey(char, color, hover) {
			this.keyboardQwerty.forEach((line) => {
				const search = line.find((key) => key.letter === char);
				if (search) {
					search.color = color;
					search.hover = hover;
				}
			});
		},
	},
};
</script>

<style lang="scss">
.layout {
	margin-left: auto !important;
	margin-right: auto !important;
	width: 90%;
	max-width: 50rem;
	display: flex;
	flex-direction: column;
	height: 100vh;
}

.text {
	width: 90%;
	max-width: 26rem;
	margin-left: auto !important;
	margin-right: auto !important;
}
</style>
