<template>
	<main class="bg-true-gray-900 h-screen">
		<div class="pt-16 layout">
			<div class="flex flex-col">
				<div class="text-center font-bold text-7xl text-light-50 mb-18">
					<h1>Hangman</h1>
				</div>
				<div class="flex justify-center mb-8">
					<p class="py-1 px-4 text-purple-300 rounded-lg border-2 border-purple-300 text-3xl select-none">{{ lives }} lives left</p>
				</div>
				<div class="flex justify-center" v-if="guessStatus">
					<h2 class="py-1 px-4 rounded-lg bg-purple-600 text-light-50 text-center font-bold text-4xl tracking-widest select-none">{{ guessStatus.toUpperCase() }}</h2>
				</div>
				<div class="flex justify-center content-center items-center my-16">
					<p class="text-center font-bold text-2xl text-light-900">{{ msg }}</p>
				</div>
			</div>
			<div class="flex flex-col justify-center mt-auto mb-8">
				<ul
					v-for="line in keyboardQwerty"
					:key="line"
					class="inline-flex flex-row justify-center not-last:pb-1"
				>
					<li v-for="key in line" :key="key.letter" class="not-last:pr-1">
						<Char @click="guessLetter(key.letter)" :class="key.color">{{ key.letter.toUpperCase() }}</Char>
					</li>
				</ul>
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
			msg: '',
			guessStatus: null,
			keyboardQwerty: [
				[
					{
						letter: "q",
						color: "bg-true-gray-700"
					},
					{
						letter: "w",
						color: "bg-true-gray-700"
					},
					{
						letter: "e",
						color: "bg-true-gray-700"
					},
					{
						letter: "r",
						color: "bg-true-gray-700"
					},
					{
						letter: "t",
						color: "bg-true-gray-700"
					},
					{
						letter: "y",
						color: "bg-true-gray-700"
					},
					{
						letter: "u",
						color: "bg-true-gray-700"
					},
					{
						letter: "i",
						color: "bg-true-gray-700"
					},
					{
						letter: "o",
						color: "bg-true-gray-700"
					},
					{
						letter: "p",
						color: "bg-true-gray-700"
					}
				], [
					{
						letter: "a",
						color: "bg-true-gray-700"
					},
					{
						letter: "s",
						color: "bg-true-gray-700"
					},
					{
						letter: "d",
						color: "bg-true-gray-700"
					},
					{
						letter: "f",
						color: "bg-true-gray-700"
					},
					{
						letter: "g",
						color: "bg-true-gray-700"
					},
					{
						letter: "h",
						color: "bg-true-gray-700"
					},
					{
						letter: "j",
						color: "bg-true-gray-700"
					},
					{
						letter: "k",
						color: "bg-true-gray-700"
					},
					{
						letter: "l",
						color: "bg-true-gray-700"
					},
					{
						letter: "p",
						color: "bg-true-gray-700"
					}
				], [
					{
						letter: "z",
						color: "bg-true-gray-700"
					},
					{
						letter: "x",
						color: "bg-true-gray-700"
					},
					{
						letter: "c",
						color: "bg-true-gray-700"
					},
					{
						letter: "v",
						color: "bg-true-gray-700"
					},
					{
						letter: "b",
						color: "bg-true-gray-700"
					},
					{
						letter: "n",
						color: "bg-true-gray-700"
					},
					{
						letter: "m",
						color: "bg-true-gray-700"
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
	methods: {
		guessLetter(char) {
			if (this.game.is_alive()) {
				const lives = this.lives;
				this.msg = this.game.validate_user_guess(char);
				this.guessStatus = this.game.get_guess_status();
				this.lives = this.game.get_lives();
				if (this.lives !== lives) {
					this.searchForKey(char, "bg-red-700")
				} else {
					this.searchForKey(char, "bg-emerald-700")
				}
			} else {
				this.msg = "The game is over!";
			}
		},
		searchForKey(char, color) {
			this.keyboardQwerty.forEach((line) => {
				const search = line.find((key) => key.letter === char);
				if (search) {
					search.color = color;
				}
			});
		}
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
</style>
