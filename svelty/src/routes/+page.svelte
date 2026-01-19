<link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/4.7.0/css/font-awesome.min.css">

<script>
    import { goto } from '$app/navigation';

    function goToPage(param) {
        goto(`/${param}`);
    }
    
    // Static assets in SvelteKit are served from the static folder at root
    // The file should be at static/images/nasa.png
    const nasaImage = '/images/nasa.png';
</script>

<div class="main-container">
    <div class="button-grid">
        <button class="tft-button" on:click={() => goToPage('tft')}>
            <i class="fa fa-music"></i>
            <span>TFT</span>
        </button>
        <button class="nasa-button" style="background-image: url('{nasaImage}')" on:click={() => goToPage('nasa')}>
            <i class="fa fa-globe"></i>
            <span>NASA</span>
        </button>
        <button class="meal-button" on:click={() => goToPage('meal')}>
            <i class="fa fa-cutlery"></i>
            <span>MEAL</span>
        </button>
        <button class="geo-button" on:click={() => goToPage('geo')}>
            <i class="fa fa-map"></i>
            <span>GEO</span>
        </button>
    </div>
</div>

<style>
    .main-container {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 100vh;
        width: 100vw;
        padding: 20px;
        box-sizing: border-box;
        background: linear-gradient(135deg, #5e7495 0%, #4a6797 100%);
    }

    .button-grid {
        display: grid;
        grid-template-columns: repeat(2, 1fr);
        grid-template-rows: repeat(2, 1fr);
        gap: 30px;
        width: 100%;
        max-width: 550px;
        height: 90vh;
        max-height: 900px;
    }

    button {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 0;
        color: white;
        border: none;
        border-radius: 30px;
        transition: all 0.2s ease;
        font-size: 2.5rem;
        font-weight: bold;
        cursor: pointer;
        min-height: 200px;
        box-shadow: 0 8px 16px rgba(0, 0, 0, 0.3);
        -webkit-tap-highlight-color: transparent;
        touch-action: manipulation;
    }

    button i {
        font-size: 4rem;
        margin-bottom: 15px;
    }

    button span {
        font-size: 2rem;
        font-weight: 600;
        text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.5);
    }

    button:active {
        transform: scale(0.95);
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
    }

    button:disabled {
        background-color: #d6d6d6;
        cursor: not-allowed;
        opacity: 0.6;
    }

    .tft-button {
        background-color: hsl(24, 100%, 50%);
        grid-column: 1 / 2;
        grid-row: 1 / 2;
    }

    .tft-button:active {
        background-color: hsl(24, 100%, 45%);
    }

    .nasa-button {
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        background-size: cover;
        background-position: center;
        background-blend-mode: overlay;
        grid-column: 2 / 3;
        grid-row: 1 / 2;
    }

    .nasa-button:active {
        opacity: 0.9;
    }

    .meal-button {
        background-color: #7700ff;
        grid-column: 1 / 2;
        grid-row: 2 / 3;
    }

    .meal-button:active {
        background-color: #6600dd;
    }

    .geo-button {
        background-color: #4CAF50;
        grid-column: 2 / 3;
        grid-row: 2 / 3;
    }

    .geo-button:active {
        background-color: #45a049;
    }

    /* Landscape orientation for 7-inch screens */
    @media (orientation: landscape) {
        .button-grid {
            grid-template-columns: repeat(4, 1fr);
            grid-template-rows: 1fr;
            height: auto;
            max-height: 80vh;
            gap: 20px;
        }

        .tft-button {
            grid-column: 1 / 2;
            grid-row: 1 / 2;
        }

        .nasa-button {
            grid-column: 2 / 3;
            grid-row: 1 / 2;
        }

        .meal-button {
            grid-column: 3 / 4;
            grid-row: 1 / 2;
        }

        .geo-button {
            grid-column: 4 / 5;
            grid-row: 1 / 2;
        }

        button {
            min-height: 250px;
        }

        button i {
            font-size: 3.5rem;
        }

        button span {
            font-size: 1.8rem;
        }
    }

    /* 7-inch screen optimization (800x480) */
    @media (max-width: 800px) and (max-height: 480px) {
        .main-container {
            padding: 10px;
        }

        .button-grid {
            grid-template-columns: repeat(4, 1fr);
            grid-template-rows: 1fr;
            gap: 10px;
            height: auto;
            max-height: 100%;
            max-width: 100%;
        }

        button {
            min-height: 120px;
            padding: 10px;
        }

        button i {
            font-size: 2.5rem;
            margin-bottom: 5px;
        }

        button span {
            font-size: 1.3rem;
        }
    }

</style>