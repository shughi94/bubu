<script>
    import { goto } from '$app/navigation';

    function goToPage(param) {
        goto(`/${param}`);
    }
</script>

<style>
    button {
        padding: 10px 20px;
        font-size: 16px;
        color: white;
        border: 3px;
        border-radius: 5px;
        cursor: pointer;
        transition: background-color 0.3s ease;
    }

    button:hover:enabled {
        background-color: #0056b3;
    }

    button:active:enabled {
        background-color: #004085;
    }

    button:disabled {
        background-color: #d6d6d6;
        cursor: not-allowed;
    }

    .tft-button {background-color: hsl(24, 100%, 50%);}
    .daily-button {background-color: #00ff66;}
    .nasa-button {background-color: #0800ff;}
    .meal-button {background-color: #7700ff;}

</style>


<button class="tft-button" on:click={() => goToPage('tft')}>TFT_MIXER</button>
<button class="daily-button" on:click={() => goToPage('daily_api')}>Daily_API</button>
<button class="nasa-button" on:click={() => goToPage('nasa')}>NASA</button>
<button class="meal-button" on:click={() => goToPage('meal')}>MEAL</button>