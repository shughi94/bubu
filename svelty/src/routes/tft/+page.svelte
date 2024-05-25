<h1>CIAO123</h1>
<script>
    import { goto } from '$app/navigation';

    function goToPage() {
        goto(`/`);
    }

	let total = 0;
    async function add(param) {
		const response = await fetch('/api/tft', {
			method: 'POST',
			body: JSON.stringify({"param": param}),
			headers: {
				'content-type': 'application/json'
			}
		});

		total = await response.json();
	}

</script>

<button on:click={add("random")}>RANDOM</button>
<button on:click={add("country")}>COUNTRY</button>

<button on:click={goToPage}>BACK</button>
