<script>
    import { goto } from '$app/navigation';

	function goToPage() {
		goto(`/`);
	}
</script>

<button class="back-button" on:click={goToPage}>
	<i class="fa fa-arrow-left"></i>
</button>

<style>
	.back-button {
	position: sticky;
	top: 0;
	left: 0;
	z-index: 100;
	margin: 10px;
	padding: 8px 15px;
	background-color: rgba(11, 11, 11, 0.95);
	color: white;
	border: 2px solid white;
	border-radius: 80%;
	cursor: pointer;
	-webkit-tap-highlight-color: transparent;
	touch-action: manipulation;
	min-height: 80px;
	min-width: 80px;
	display: flex;
	align-items: center;
	justify-content: center;
	gap: 8px;
	box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
	transition: all 0.2s ease;
	align-self: flex-start;
}

.back-button:active {
	transform: scale(0.95);
	background-color: rgba(60, 60, 60, 0.95);
}

.back-button i {
	font-size: 3rem;
}
	
</style>