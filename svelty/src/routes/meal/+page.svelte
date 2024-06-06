<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Image from '../../blocks/Image.svelte';

	function goToPage() {
		goto(`/`);
	}

	let data;
	let error;
	let loading;

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:3000/meal/random');
			console.log(response);
			if (!response.ok) {
				throw new Error('Failed to fetch data');
				error = true;
			}
			data = await response.json();
		} catch (err) {
			error = err.message;
		} finally {
			loading = false;
		}
	});
</script>

<div class="container">
	{#if loading}
		<p class="loading">Loading...</p>
	{:else if error}
		<p class="error">Error: {error}</p>
	{:else if data}
		<div class="data-field">
			<h2>{data.strMeal}</h2>
		</div>
		<div class="data-field">
			<p>{data.strInstructions}</p>
		</div>
		<div class="thumbnail">
			<Image src={data.strMealThumb} alt="potd" />
		</div>
	{/if}
	<button on:click={goToPage}>BACK</button>
</div>

<style>
	.container {
		margin: auto;
		justify-content: center;
		align-items: center;
		padding: 3%;
		width: 80%;
		background-color:rgb(184, 184, 184);
		border:3px solid black;
		border-radius: 5px;
	}

	.error {
		color: red;
	}

	.loading {
		font-size: 18px;
	}

	.data-field h2 {
		font-weight: bold;
	}

</style>
