import requests, os
from config import Secrets

POTD_URL=f"https://api.nasa.gov/planetary/apod?api_key={Secrets.NASA_API_KEY}"
PATH="../rusty/ayylmao/potd.json"

if os.path.exists(PATH):
    os.remove(PATH)

print(f"Getting image of the day from nasa...")

r = requests.get(POTD_URL, stream=True)
if r.status_code == 200:
    with open(PATH, 'wb') as f:
        for chunk in r.iter_content(1024):
            f.write(chunk)

print("Done getting image of the day from nasa.")