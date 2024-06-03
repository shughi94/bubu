<script>
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';

	function goToPage() {
		goto(`/`);
	}

	let data;

	onMount(async () => {
		try {
			const response = await fetch('http://localhost:3000/meal/random');
			console.log(response);
			if (!response.ok) {
				throw new Error('Failed to fetch data');
			}
			data = await response.json()['meals'][0];
			
		} catch (err) {
			error = err.message;
		} finally {
			loading = false;
		}
	});
</script>

{#if data}
<div>
	{data.strMeal}
</div>
{/if}
<button on:click={goToPage}>BACK</button>


<style>

</style>
