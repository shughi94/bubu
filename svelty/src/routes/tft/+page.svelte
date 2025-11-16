<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
    import { goto } from '$app/navigation';

    function goToPage() {
        goto(`/`);
    }

	let total = 0;
    async function add(param) {
		// const response = await fetch('/api/tft', {
		// 	method: 'POST',
		// 	body: JSON.stringify({"param": param}),
		// 	headers: {
		// 		'content-type': 'application/json'
		// 	}
		// });

		// total = await response.json();
		let bubu = 5
	}

</script>

<div class="page-container">
	<button class="back-button" on:click={goToPage}>
		<i class="fa fa-arrow-left"></i> BACK
	</button>
	<div class="content-wrapper">
		<h1>TFT</h1>
		<div class="button-group">
			<button class="action-button random-button" on:click={() => add("random")}>
				<i class="fa fa-random"></i>
				<span>RANDOM</span>
			</button>
			<button class="action-button country-button" on:click={() => add("country")}>
				<i class="fa fa-globe"></i>
				<span>COUNTRY</span>
			</button>
		</div>
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
		padding: 30px 15px;
		gap: 40px;
		max-width: 100%;
		box-sizing: border-box;
	}

	h1 {
		margin: 0;
		font-size: 3rem;
		font-weight: bold;
		color: white;
		text-align: center;
	}

	.button-group {
		display: flex;
		flex-direction: column;
		gap: 25px;
		width: 100%;
		max-width: 500px;
	}

	.action-button {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		padding: 30px;
		color: white;
		border: none;
		border-radius: 25px;
		font-size: 1.8rem;
		font-weight: bold;
		cursor: pointer;
		min-height: 180px;
		box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		transition: all 0.2s ease;
		gap: 15px;
	}

	.action-button i {
		font-size: 3.5rem;
	}

	.action-button span {
		font-size: 1.8rem;
		font-weight: 600;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
	}

	.action-button:active {
		transform: scale(0.95);
		box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
	}

	.random-button {
		background-color: hsl(24, 100%, 50%);
	}

	.random-button:active {
		background-color: hsl(24, 100%, 45%);
	}

	.country-button {
		background-color: #4CAF50;
	}

	.country-button:active {
		background-color: #45a049;
	}

	/* Landscape orientation adjustments */
	@media (orientation: landscape) {
		.content-wrapper {
			flex-direction: row;
			gap: 30px;
		}

		.button-group {
			flex-direction: row;
			max-width: 80%;
		}

		h1 {
			font-size: 2.5rem;
		}

		.action-button {
			min-height: 200px;
			flex: 1;
		}

		.action-button i {
			font-size: 3rem;
		}

		.action-button span {
			font-size: 1.6rem;
		}
	}

	/* Smaller screens */
	@media (max-height: 800px) {
		h1 {
			font-size: 2.5rem;
		}

		.action-button {
			min-height: 150px;
			padding: 25px;
		}

		.action-button i {
			font-size: 3rem;
		}

		.action-button span {
			font-size: 1.6rem;
		}
	}
</style>
