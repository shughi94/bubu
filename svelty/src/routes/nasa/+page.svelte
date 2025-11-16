<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

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

<div class="page-container">
	<button class="back-button" on:click={goToPage}>
		<i class="fa fa-arrow-left"></i> BACK
	</button>
	<div class="content-wrapper">
		{#if data}
			<div class="title-field">
				<h1>{data.title}</h1>
			</div>
			<div class="media-field">
				{#if loading}
					<div class="loading-container">
						<p class="loading">Loading...</p>
					</div>
				{:else if error}
					<div class="error-container">
						<p class="error">Error: {error}</p>
					</div>
				{:else if data.media_type == 'video'}
					<div class="video-container">
						<Youtube id={video_id}></Youtube>
					</div>
				{:else if data.media_type == 'image' && data && data.hdurl}
					<div class="image-container">
						<Image src={data.hdurl} alt="NASA Picture of the Day" />
					</div>
				{:else}
					<div class="error-container">
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
	.page-container {
		min-height: 100vh;
		width: 100vw;
		display: flex;
		flex-direction: column;
		background-color: #2c2c2c;
		overflow-y: auto;
		overflow-x: hidden;
		-webkit-overflow-scrolling: touch;
	}

	.back-button {
		position: sticky;
		top: 0;
		z-index: 100;
		margin: 15px;
		padding: 15px 25px;
		background-color: rgba(44, 44, 44, 0.95);
		color: white;
		border: 2px solid white;
		border-radius: 25px;
		font-size: 1.4rem;
		font-weight: 600;
		cursor: pointer;
		backdrop-filter: blur(10px);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		min-height: 50px;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
		transition: all 0.2s ease;
	}

	.back-button:active {
		transform: scale(0.95);
		background-color: rgba(60, 60, 60, 0.95);
	}

	.back-button i {
		font-size: 1.2rem;
	}

	.content-wrapper {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 15px;
		gap: 20px;
		max-width: 100%;
		box-sizing: border-box;
	}

	.title-field {
		width: 100%;
		padding: 15px;
		background-color: rgba(184, 184, 184, 0.1);
		border-radius: 15px;
		text-align: center;
	}

	.title-field h1 {
		margin: 0;
		font-size: 1.6rem;
		font-weight: bold;
		color: white;
		line-height: 1.4;
	}

	.media-field {
		width: 100%;
		flex: 1;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		min-height: 300px;
		max-height: 60vh;
	}

	.image-container,
	.video-container {
		width: 100%;
		height: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 15px;
		overflow: hidden;
		background-color: rgba(184, 184, 184, 0.1);
	}

	.image-container :global(img) {
		max-width: 100%;
		max-height: 60vh;
		object-fit: contain;
		border-radius: 15px;
	}

	.video-container :global(iframe) {
		width: 100%;
		height: 100%;
		min-height: 300px;
		max-height: 60vh;
		border-radius: 15px;
	}

	.loading-container,
	.error-container {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		height: 100%;
		min-height: 200px;
		padding: 30px;
	}

	.loading {
		font-size: 1.8rem;
		color: white;
	}

	.error {
		color: #ff4444;
		font-size: 1.5rem;
		text-align: center;
	}

	.info-field {
		width: 100%;
		padding: 20px;
		background-color: rgba(184, 184, 184, 0.1);
		border-radius: 15px;
		margin-bottom: 20px;
	}

	.info-field p {
		margin: 0;
		font-size: 1.1rem;
		line-height: 1.6;
		color: white;
		text-align: justify;
	}

	/* Landscape orientation adjustments */
	@media (orientation: landscape) {
		.media-field {
			max-height: 70vh;
		}

		.image-container :global(img) {
			max-height: 70vh;
		}

		.video-container :global(iframe) {
			max-height: 70vh;
		}

		.title-field h1 {
			font-size: 1.4rem;
		}

		.info-field p {
			font-size: 1rem;
		}
	}

	/* Smaller screens */
	@media (max-height: 800px) {
		.media-field {
			max-height: 50vh;
		}

		.title-field h1 {
			font-size: 1.4rem;
		}

		.info-field {
			padding: 15px;
		}

		.info-field p {
			font-size: 1rem;
		}
	}
</style>
