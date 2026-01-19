<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
	import BackButton from '../../components/backButton.svelte';

	let audioElement;
	let isPlaying = false;
	
	// Serve from static folder
	const audioUrl = '/tft_song.ogg';

	function togglePlay() {
		if (audioElement) {
			if (isPlaying) {
				audioElement.pause();
			} else {
				audioElement.play();
			}
			isPlaying = !isPlaying;
		}
	}

	function handlePlay() {
		isPlaying = true;
	}

	function handlePause() {
		isPlaying = false;
	}
</script>

<div class="page-container">
	<BackButton />
	<div class="content-wrapper">
		<audio 
			bind:this={audioElement}
			src={audioUrl}
			on:play={handlePlay}
			on:pause={handlePause}
		></audio>
		<button class="play-button" on:click={togglePlay}>
			{#if isPlaying}
				<i class="fa fa-pause"></i>
			{:else}
				<i class="fa fa-play"></i>
			{/if}
		</button>
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

	.content-wrapper {
		flex: 1;
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 30px 15px;
		max-width: 100%;
		box-sizing: border-box;
	}

	.play-button {
		width: 300px;
		height: 300px;
		border-radius: 50%;
		border: none;
		background-color: #4CAF50;
		color: white;
		font-size: 6rem;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		transition: all 0.2s ease;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
	}

	.play-button:hover {
		background-color: #45a049;
		transform: scale(1.05);
	}

	.play-button:active {
		transform: scale(0.95);
		background-color: #3d8b40;
	}

	/* Landscape orientation adjustments */
	@media (orientation: landscape) {
		.play-button {
			width: 250px;
			height: 250px;
			font-size: 5rem;
		}
	}

	/* 7-inch screen optimization (800x480) */
	@media (max-width: 800px) and (max-height: 480px) {
		.content-wrapper {
			padding: 10px;
		}

		.play-button {
			width: 200px;
			height: 200px;
			font-size: 4rem;
		}
	}

</style>
