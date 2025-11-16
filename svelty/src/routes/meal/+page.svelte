<script>
	import { onMount } from 'svelte';
	import Image from '../../blocks/Image.svelte';
	import BackButton from '../../components/backButton.svelte';

	let data;
	let error;
	let loading = true;

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

<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<div class="page-container">
	<BackButton />
	<div class="content-wrapper">
		{#if loading}
			<div class="loading-container">
				<p class="loading">Loading...</p>
			</div>
		{:else if error}
			<div class="error-container">
				<p class="error">Error: {error}</p>
			</div>
		{:else if data}
			<div class="meal-header">
				<h1>{data.strMeal}</h1>
			</div>
			<div class="image-section">
				<Image src={data.strMealThumb} alt={data.strMeal} />
			</div>
			<div class="instructions-section">
				<h2>Instructions</h2>
				<p>{data.strInstructions}</p>
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


	.content-wrapper {
		flex: 1;
		display: flex;
		flex-direction: column;
		padding: 15px;
		gap: 20px;
		max-width: 100%;
		box-sizing: border-box;
	}

	.meal-header {
		width: 100%;
		padding: 20px;
		background-color: rgba(119, 0, 255, 0.2);
		border-radius: 15px;
		text-align: center;
	}

	.meal-header h1 {
		margin: 0;
		font-size: 2rem;
		font-weight: bold;
		color: white;
		line-height: 1.3;
	}

	.image-section {
		width: 100%;
		display: flex;
		justify-content: center;
		align-items: center;
		border-radius: 15px;
		overflow: hidden;
		background-color: rgba(184, 184, 184, 0.1);
		padding: 10px;
	}

	.image-section :global(img) {
		max-width: 100%;
		max-height: 400px;
		object-fit: contain;
		border-radius: 15px;
	}

	.instructions-section {
		width: 100%;
		padding: 20px;
		background-color: rgba(184, 184, 184, 0.1);
		border-radius: 15px;
		margin-bottom: 20px;
	}

	.instructions-section h2 {
		margin: 0 0 15px 0;
		font-size: 1.6rem;
		font-weight: bold;
		color: white;
	}

	.instructions-section p {
		margin: 0;
		font-size: 1.1rem;
		line-height: 1.7;
		color: white;
		text-align: justify;
		white-space: pre-line;
	}

	.loading-container,
	.error-container {
		display: flex;
		justify-content: center;
		align-items: center;
		width: 100%;
		min-height: 60vh;
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

	/* Landscape orientation adjustments */
	@media (orientation: landscape) {
		.meal-header h1 {
			font-size: 1.6rem;
		}

		.image-section :global(img) {
			max-height: 300px;
		}

		.instructions-section h2 {
			font-size: 1.4rem;
		}

		.instructions-section p {
			font-size: 1rem;
		}
	}

	/* Smaller screens */
	@media (max-height: 800px) {
		.meal-header {
			padding: 15px;
		}

		.meal-header h1 {
			font-size: 1.6rem;
		}

		.image-section :global(img) {
			max-height: 300px;
		}

		.instructions-section {
			padding: 15px;
		}

		.instructions-section h2 {
			font-size: 1.4rem;
		}

		.instructions-section p {
			font-size: 1rem;
		}
	}
</style>
