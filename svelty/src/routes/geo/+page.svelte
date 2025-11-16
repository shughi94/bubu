<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	function goToPage() {
		goto(`/`);
	}

	let data;
	let error;
	let loading = true;

	async function fetchGeo() {
		loading = true;
		error = null;
		try {
			const response = await fetch('http://localhost:3000/geo/random');
			if (!response.ok) {
				throw new Error('Failed to fetch data');
			}
			data = await response.json();
		} catch (err) {
			error = err.message;
		} finally {
			loading = false;
		}
	}

	onMount(async () => {
		await fetchGeo();
	});
</script>

<div class="page-container">
	<button class="back-button" on:click={goToPage}>
		<i class="fa fa-arrow-left"></i> BACK
	</button>
	<div class="content-wrapper">
		{#if loading}
			<div class="loading-container">
				<p class="loading">Loading...</p>
			</div>
		{:else if error}
			<div class="error-container">
				<p class="error">Error: {error}</p>
				<button class="retry-button" on:click={fetchGeo}>Retry</button>
			</div>
		{:else if data}
			<div class="geo-header">
				<h1>Random Country</h1>
			</div>
			<div class="geo-card">
				<div class="info-section">
					<div class="info-item">
						<i class="fa fa-globe"></i>
						<div class="info-content">
							<label>Country</label>
							<h2>{data.country}</h2>
						</div>
					</div>
					<div class="info-item">
						<i class="fa fa-map-marker"></i>
						<div class="info-content">
							<label>Capital</label>
							<h2>{data.capital}</h2>
						</div>
					</div>
				</div>
				<button class="refresh-button" on:click={fetchGeo}>
					<i class="fa fa-refresh"></i>
					<span>Get Another Country</span>
				</button>
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
		align-items: center;
		justify-content: center;
		padding: 20px;
		gap: 30px;
		max-width: 100%;
		box-sizing: border-box;
	}

	.geo-header {
		width: 100%;
		max-width: 500px;
		padding: 20px;
		text-align: center;
	}

	.geo-header h1 {
		margin: 0;
		font-size: 2.5rem;
		font-weight: bold;
		color: white;
	}

	.geo-card {
		width: 100%;
		max-width: 500px;
		padding: 30px;
		background-color: rgba(184, 184, 184, 0.15);
		border-radius: 25px;
		display: flex;
		flex-direction: column;
		gap: 30px;
		box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
	}

	.info-section {
		display: flex;
		flex-direction: column;
		gap: 25px;
	}

	.info-item {
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 20px;
		background-color: rgba(255, 255, 255, 0.1);
		border-radius: 20px;
	}

	.info-item i {
		font-size: 2.5rem;
		color: #4CAF50;
		min-width: 50px;
		text-align: center;
	}

	.info-content {
		flex: 1;
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	.info-content label {
		font-size: 0.9rem;
		color: rgba(255, 255, 255, 0.7);
		text-transform: uppercase;
		letter-spacing: 1px;
		font-weight: 600;
	}

	.info-content h2 {
		margin: 0;
		font-size: 2rem;
		font-weight: bold;
		color: white;
	}

	.refresh-button {
		padding: 20px 30px;
		background-color: #4CAF50;
		color: white;
		border: none;
		border-radius: 20px;
		font-size: 1.4rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 15px;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		transition: all 0.2s ease;
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
		min-height: 60px;
	}

	.refresh-button:active {
		transform: scale(0.95);
		background-color: #45a049;
	}

	.refresh-button i {
		font-size: 1.5rem;
	}

	.loading-container,
	.error-container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		width: 100%;
		min-height: 60vh;
		padding: 30px;
		gap: 20px;
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

	.retry-button {
		padding: 15px 30px;
		background-color: #ff4444;
		color: white;
		border: none;
		border-radius: 15px;
		font-size: 1.2rem;
		font-weight: 600;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		transition: all 0.2s ease;
	}

	.retry-button:active {
		transform: scale(0.95);
		background-color: #cc3333;
	}

	/* Landscape orientation adjustments */
	@media (orientation: landscape) {
		.geo-header h1 {
			font-size: 2rem;
		}

		.info-content h2 {
			font-size: 1.6rem;
		}

		.refresh-button {
			font-size: 1.2rem;
			padding: 15px 25px;
		}
	}

	/* Smaller screens */
	@media (max-height: 800px) {
		.geo-header h1 {
			font-size: 2rem;
		}

		.geo-card {
			padding: 20px;
		}

		.info-content h2 {
			font-size: 1.6rem;
		}

		.info-item i {
			font-size: 2rem;
		}
	}
</style>

