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

<button class="back-button" on:click={goToPage}>BACK</button>
<div class="layout">
	<div class="container">
		{#if data}
			<div class="title-field">
				<h2>{data.title}</h2>
			</div>
			<div class="image-field">
				{#if loading}
					<p class="loading">Loading...</p>
				{:else if error}
					<p class="error">Error: {error}</p>
				{:else if data.media_type == 'video'}
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
			</div>
			<div class="info-field">
				<p>{data.explanation}</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.layout {
		height: 100%;
		width: 100%;
		display: grid;
		grid:
			'title-field' 10%
			'image-field' 70%
			'info-field' auto
			/ 1fr;
		gap: 5px;
	}

	.back-button {
		margin-left: 2%;
		margin-top: 2%;
		padding: 5px;
	}

	.fullscreen-image {
		
	}

	.container {
		margin: auto;
		justify-content: center;
		align-items: center;
		padding: 3%;
		width: 80%;
		background-color: rgb(184, 184, 184);
		border: 3px solid black;
		border-radius: 5px;
	}

	.error {
		color: red;
	}

	.loading {
		font-size: 18px;
	}

	.image-field {
		grid-area: image-field;
	}

	.title-field {
		font-weight: bold;
		grid-area: title-field;
		text-align: center;
	}

	.info-field {
		font-size: medium;
		grid-area: info-field;
	}
</style>
