<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
    import { goto } from '$app/navigation';

    function goToPage(param) {
        goto(`/${param}`);
    }
</script>

<style>
    button {
        padding: 50px 100px;
        color: white;
        border: 5px;
        border-radius: 80%;
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
    /* .daily-button {background-color: #00ff66;} */
    .nasa-button {background-image: url("public/images/nasa.png");}
    .meal-button {background-color: #7700ff;}

</style>


<button class="tft-button fa fa-music" on:click={() => goToPage('tft')}></button>
<!-- <button class="daily-button" on:click={() => goToPage('daily_api')}>Daily_API</button> -->
<button class="nasa-button fa-globe" aria-hidden="true" on:click={() => goToPage('nasa')}></button>
<button class="meal-button fa fa-cutlery" on:click={() => goToPage('meal')}></button>