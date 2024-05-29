<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import Youtube from 'svelte-youtube-embed';
	import Image from '../../blocks/Image.svelte';

	function goToPage() {
		goto(`/`);
	}

	let data = null;
	let video_id = null;
	let image_url = null;
	let error = null;
	let loading = true;

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:3000/nasa/potd');
			console.log(response);
			if (!response.ok) {
				throw new Error('Failed to fetch data');
			}
			data = await response.json();
			video_id = data.url.replace('https://www.youtube.com/embed/', '').replace('?rel=0', '');

			if (data.hdurl) {
				image_url = data.hdurl;
			} else {
				image_url = data.url;
			}
		} catch (err) {
			error = err.message;
		} finally {
			loading = false;
		}
	});
</script>

{#if data}
{#if data.media_type == 'video'}
	<div>
		<Youtube id={video_id}></Youtube>
	</div>
{:else if data.media_type == 'image' && data && data.hdurl}
	<div class="fullscreen-image"><Image src={data.hdurl} alt="potd" /></div>
{:else}
	<div class="data-field">
		<p>No support found for: {data.media_type}</p>
	</div>
{/if}
{/if}
<div class="container">
	{#if loading}
		<p class="loading">Loading...</p>
	{:else if error}
		<p class="error">Error: {error}</p>
	{:else}
		<div class="data-field">
			<h2>{data.title}</h2>
		</div>
		<div class="data-field">
			<p>{data.explanation}</p>
		</div>
	{/if}
	<button on:click={goToPage}>BACK</button>
</div>

<style>
	.fullscreen-image {
		padding-top:5px;
		display: flex;
		justify-content: center;
		margin: auto;
  		align-items: center;
        width: 85%;
		height: auto;
		padding-bottom: 2%;
	}

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

	.data-field p {
	}
</style>
