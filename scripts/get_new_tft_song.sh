cd ../../tft-mixer
source .venv/bin/activate
python3 main.py --pogs --only_file
cp final.ogg ../bubu/svelty/static/tft_song.ogg
deactivate