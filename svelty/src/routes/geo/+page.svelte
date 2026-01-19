<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
	import { onMount } from 'svelte';
	import BackButton from '../../components/backButton.svelte';

	let data;
	let error;
	let loading = true;
	let selectedAnswer = null;
	let showResult = false;
	let isCorrect = false;

	async function fetchGeo() {
		loading = true;
		error = null;
		selectedAnswer = null;
		showResult = false;
		isCorrect = false;
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

	function selectAnswer(capital) {
		if (showResult) return;
		
		selectedAnswer = capital;
		isCorrect = capital === data.capital;
		showResult = true;
	}

	function nextQuestion() {
		fetchGeo();
	}

	onMount(async () => {
		await fetchGeo();
	});
</script>

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
				<button class="retry-button" on:click={fetchGeo}>Retry</button>
			</div>
		{:else if data}
			<div class="quiz-header">
				<h1>Geography Quiz</h1>
				<div class="question">
					<p class="question-text">What is the capital of</p>
					<h2 class="country-name">{data.country}?</h2>
				</div>
			</div>
			<div class="quiz-content">
				<div class="choices-grid">
					{#each data.choices as choice, index}
						<button
							class="choice-button"
							class:correct={showResult && choice === data.capital}
							class:wrong={showResult && selectedAnswer === choice && choice !== data.capital}
							class:disabled={showResult}
							on:click={() => selectAnswer(choice)}
							disabled={showResult}
						>
							<span class="choice-label">{String.fromCharCode(65 + index)}.</span>
							<span class="choice-text">{choice}</span>
							{#if showResult && choice === data.capital}
								<i class="fa fa-check"></i>
							{/if}
							{#if showResult && selectedAnswer === choice && choice !== data.capital}
								<i class="fa fa-times"></i>
							{/if}
						</button>
					{/each}
				</div>

				{#if showResult}
					<div class="result-section">
						{#if isCorrect}
							<div class="result-message correct">
								<i class="fa fa-check-circle"></i>
								<p>Correct!</p>
							</div>
						{:else}
							<div class="result-message wrong">
								<i class="fa fa-times-circle"></i>
								<p>Wrong! The correct answer is <strong>{data.capital}</strong></p>
							</div>
						{/if}
						<button class="next-button" on:click={nextQuestion}>
							<i class="fa fa-arrow-right"></i>
							<span>Next Question</span>
						</button>
					</div>
				{/if}
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
		align-items: center;
		justify-content: flex-start;
		padding: 20px;
		gap: 25px;
		max-width: 100%;
		box-sizing: border-box;
	}

	.quiz-header {
		width: 100%;
		max-width: 600px;
		padding: 20px;
		text-align: center;
	}

	.quiz-header h1 {
		margin: 0 0 20px 0;
		font-size: 2.2rem;
		font-weight: bold;
		color: white;
	}

	.question {
		background-color: rgba(76, 175, 80, 0.2);
		border-radius: 20px;
		padding: 25px;
	}

	.question-text {
		margin: 0 0 10px 0;
		font-size: 1.3rem;
		color: rgba(255, 255, 255, 0.9);
	}

	.country-name {
		margin: 0;
		font-size: 2.2rem;
		font-weight: bold;
		color: white;
	}

	.quiz-content {
		width: 100%;
		max-width: 600px;
		display: flex;
		flex-direction: column;
		gap: 25px;
	}

	.choices-grid {
		display: grid;
		grid-template-columns: 1fr;
		gap: 15px;
		width: 100%;
	}

	.choice-button {
		padding: 20px 25px;
		background-color: rgba(255, 255, 255, 0.1);
		color: white;
		border: 3px solid rgba(255, 255, 255, 0.3);
		border-radius: 15px;
		font-size: 1.4rem;
		font-weight: 600;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 15px;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		transition: all 0.2s ease;
		min-height: 70px;
		text-align: left;
	}

	.choice-button:not(.disabled):hover {
		background-color: rgba(255, 255, 255, 0.15);
		border-color: rgba(255, 255, 255, 0.5);
	}

	.choice-button:not(.disabled):active {
		transform: scale(0.98);
	}

	.choice-button.correct {
		background-color: rgba(76, 175, 80, 0.3);
		border-color: #4CAF50;
		color: white;
	}

	.choice-button.wrong {
		background-color: rgba(244, 67, 54, 0.3);
		border-color: #f44336;
		color: white;
	}

	.choice-button.disabled {
		cursor: not-allowed;
		opacity: 0.7;
	}

	.choice-label {
		font-size: 1.6rem;
		font-weight: bold;
		min-width: 40px;
	}

	.choice-text {
		flex: 1;
	}

	.choice-button i {
		font-size: 1.5rem;
	}

	.result-section {
		display: flex;
		flex-direction: column;
		gap: 20px;
		width: 100%;
	}

	.result-message {
		padding: 20px;
		border-radius: 15px;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
	}

	.result-message.correct {
		background-color: rgba(76, 175, 80, 0.3);
		color: white;
	}

	.result-message.wrong {
		background-color: rgba(244, 67, 54, 0.3);
		color: white;
	}

	.result-message i {
		font-size: 3rem;
	}

	.result-message p {
		margin: 0;
		font-size: 1.4rem;
		font-weight: 600;
	}

	.result-message strong {
		font-weight: bold;
		font-size: 1.5rem;
	}

	.next-button {
		padding: 20px 30px;
		background-color: #4CAF50;
		color: white;
		border: none;
		border-radius: 15px;
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

	.next-button:active {
		transform: scale(0.95);
		background-color: #45a049;
	}

	.next-button i {
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
		.quiz-header h1 {
			font-size: 1.8rem;
		}

		.country-name {
			font-size: 1.8rem;
		}

		.choice-button {
			padding: 15px 20px;
			font-size: 1.2rem;
			min-height: 60px;
		}

		.result-message p {
			font-size: 1.2rem;
		}

		.next-button {
			font-size: 1.2rem;
			padding: 15px 25px;
		}
	}

	/* 7-inch screen optimization (800x480) */
	@media (max-width: 800px) and (max-height: 480px) {
		.content-wrapper {
			padding: 10px;
			gap: 12px;
		}

		.quiz-header {
			padding: 10px;
		}

		.quiz-header h1 {
			font-size: 1.3rem;
			margin-bottom: 10px;
		}

		.question {
			padding: 12px;
		}

		.question-text {
			font-size: 1rem;
			margin-bottom: 5px;
		}

		.country-name {
			font-size: 1.4rem;
		}

		.choices-grid {
			gap: 8px;
		}

		.choice-button {
			padding: 10px 15px;
			font-size: 1rem;
			min-height: 45px;
		}

		.choice-label {
			font-size: 1.2rem;
			min-width: 30px;
		}

		.result-section {
			gap: 12px;
		}

		.result-message {
			padding: 12px;
		}

		.result-message i {
			font-size: 2rem;
		}

		.result-message p {
			font-size: 1rem;
		}

		.result-message strong {
			font-size: 1.1rem;
		}

		.next-button {
			padding: 12px 20px;
			font-size: 1.1rem;
			min-height: 45px;
		}
	}

</style>

