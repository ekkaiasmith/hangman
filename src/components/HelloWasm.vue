<template>
	<button @click="greet('webAssembly')">hello wasm</button>
</template>

<script setup lang="ts">
import { defineComponent } from "vue";
import init, { greet, test, Hangman } from "hangman";

await init(await fetch("../../hangman/pkg/hangman_bg.wasm"));

console.log(test("bpnzoir"));
</script>
