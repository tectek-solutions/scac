### Implementing OAuth2 with FastAPI

1. **Install the required packages:**

   ```bash
   pip install requests requests_oauthlib python-dotenv
   ```
Below is the utils import for the OAuth2 implementation:

```python
from requests_oauthlib import OAuth2Session
import os
from dotenv import load_dotenv
from fastapi.responses import HTMLResponse
```

2. **Create a `.env` file in the root directory of the project:**

```bash
touch .env
CLIENT_ID=your_client_id
CLIENT_SECRET=your_client_secret
```

3. **Then create a route /login that redirects to the OAuth2 provider:**

```python
@app.get("/login")
async def login():
    authorization_url, state = oauth.authorization_url(authorization_base_url, access_type="offline", prompt="consent")
    return authorization_url
```
This route redirects the user to the OAuth2 provider's login page and the user gonna be ask to connect with his gmail and consent or not to share his information.

4. **Create a route /callback that handles the OAuth2 provider's response:**

```python
@app.get("/callback")
async def callback(request: Request):
    authorization_response = str(request.url)
    token = oauth.fetch_token(token_url, authorization_response=authorization_response, client_secret=client_secret)
    response = oauth.get('https://www.googleapis.com/oauth2/v1/userinfo?alt=json')
    if response.status_code == 200:
        user_info = response.json()
        return HTMLResponse(content=f"Token récupéré avec succès !<br>Info utilisateur : {user_info}<br>Token : {token}")
    else:
        return f"Erreur lors de la récupération des informations utilisateur: {response.status_code}"
```
When the user is redirected back to the application, the OAuth2 provider sends an authorization response to the callback route. This route fetches the token and uses it to retrieve the user's information here we display the user mail, his id his name and his picture.

5. **The token is store and ready to be used in the application.**
