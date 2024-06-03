import requests, os

MEAL_URL="https://www.themealdb.com/api/json/v1/1/random.php"
PATH="../rusty/ayylmao/random_meal.json"

if os.path.exists(PATH):
    os.remove(PATH)

print(f"Getting meal of the day...")

r = requests.get(MEAL_URL, stream=True)
if r.status_code == 200:
    with open(PATH, 'wb') as f:
        for chunk in r.iter_content(1024):
            f.write(chunk)

print("Done getting meal of the day.")