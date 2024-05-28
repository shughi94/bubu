<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Youtube from "svelte-youtube-embed";

	function goToPage() {
		goto(`/`);
	}

	let data = null;
	let video_id = null
	let thumbnailUrl = null
	let error = null;
	let loading = true;

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:3000/nasa/potd');
			console.log(response)
			if (!response.ok) {
				throw new Error('Failed to fetch data');
			}
			data = await response.json();
			video_id = data.url.replace("https://www.youtube.com/embed/", "").replace("?rel=0","");
			thumbnailUrl = "https://img.youtube.com/vi/${video_id}/maxresdefault.jpg"; //TODO?

		} catch (err) {
			error = err.message;
		} finally {
			loading = false;
		}
	});
</script>

<style>
	.container {
		max-width: 600px;
		margin: 0 auto;
		padding: 20px;
		font-family: Arial, sans-serif;
	}

	.error {
		color: red;
	}

	.loading {
		font-size: 18px;
	}

	.data-field {
		margin-bottom: 10px;
	}

	.data-field p {
		font-weight: bold;
	}

	.data-field span {
		margin-left: 10px;
	}
</style>

<h1>CIAO123</h1>
<div class="container">
	{#if loading}
		<p class="loading">Loading...</p>
	{:else if error}
		<p class="error">Error: {error}</p>
	{:else}
		<div class="data-field">
			<p>Date:</p>
			<span>{data.date}</span>
		</div>
		<div class="data-field">
			<p>Explanation:</p>
			<span>{data.explanation}</span>
		</div>
		<div class="data-field">
			<p>Title:</p>
			<span>{data.title}</span>
		</div>
		<div class="data-field">
			<Youtube id={video_id}></Youtube>
		</div>
	{/if}
</div>

<button on:click={goToPage}>BACK</button>