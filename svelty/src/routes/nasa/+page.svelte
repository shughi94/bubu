<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
	import BackButton from '../../components/backButton.svelte';
	import { onMount } from 'svelte';
	import Youtube from 'svelte-youtube-embed';
	import Image from '../../blocks/Image.svelte';



	let data = null;
	let video_id = null;
	let image_url = null;
	let error = null;
	let loading = true;
	let showControls = false;
	let mouseMoveTimeout;

	function handleMouseMove() {
		showControls = true;
		clearTimeout(mouseMoveTimeout);
		mouseMoveTimeout = setTimeout(() => {
			showControls = false;
		}, 2000);
	}

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:3000/nasa/potd');
			console.log(response);
			if (!response.ok) {
				throw new Error('Failed to fetch data');
			}
			data = await response.json();

			if(data.url)
            {
				video_id = data.url.replace('https://www.youtube.com/embed/', '').replace('?rel=0', '');
			}

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

<div class="page-container" role="main" on:mousemove={handleMouseMove} on:touchstart={handleMouseMove}>
	<div class="controls-overlay" class:visible={showControls}>
		<BackButton />
		{#if data}
			<div class="title-field">
				<h1>{data.title}</h1>
			</div>
		{/if}
	</div>
	<div class="content-wrapper">
		{#if data}
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


	.controls-overlay {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 100;
		pointer-events: none;
		opacity: 0;
		transition: opacity 0.3s ease;
	}

	.controls-overlay.visible {
		opacity: 1;
	}

	.controls-overlay :global(.back-button) {
		pointer-events: auto;
		position: absolute;
		top: 10px;
		left: 10px;
	}

	.content-wrapper {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 0;
		gap: 20px;
		max-width: 100%;
		box-sizing: border-box;
		margin-top: 0;
	}

	.title-field {
		position: absolute;
		top: 10px;
		left: 50%;
		transform: translateX(-50%);
		width: auto;
		max-width: 90%;
		padding: 8px 16px;
		background-color: rgba(0, 0, 0, 0.8);
		border: 2px solid rgba(0, 0, 0, 0.9);
		border-radius: 25px;
		text-align: center;
		box-sizing: border-box;
		pointer-events: none;
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
		min-height: 400px;
		max-height: 75vh;
		position: relative;
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
		width: 100%;
		max-width: 100%;
		object-fit: contain;
		border-radius: 15px;
	}

	.video-container :global(iframe) {
		width: 100%;
		height: 100%;
		min-height: 400px;
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
		margin-left: 15px;
		margin-right: 15px;
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
		.content-wrapper {
			margin-top: 0;
			padding-top: 20px;
		}

		.video-container :global(iframe) {
			max-height: 80vh;
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
		.content-wrapper {
			margin-top: 0;
			padding-top: 20px;
		}

		.title-field {
			padding-top: 90px;
		}

		.media-field {
			max-height: 65vh;
			min-height: 350px;
		}

		.image-container :global(img) {
			max-height: 65vh;
		}

		.video-container :global(iframe) {
			max-height: 65vh;
			min-height: 350px;
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
