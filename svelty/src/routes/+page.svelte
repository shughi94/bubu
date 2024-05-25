<script>
    import { goto } from '$app/navigation';

    function goToPage(param) {
        goto(`/${param}`);
    }
</script>


<button on:click={() => goToPage('tft')}>TFT_MIXER</button>
<button on:click={() => goToPage('daily_api')}>Daily_API</button>