from flask import Flask, request, redirect
from requests_oauthlib import OAuth2Session
import os
from dotenv import load_dotenv

# Charger le fichier .env
load_dotenv()

# Accéder aux variables d'environnement

os.environ['OAUTHLIB_INSECURE_TRANSPORT'] = '1'

app = Flask(__name__)
client_id = os.getenv('CLIENT_ID')
client_secret = os.getenv('CLIENT_SECRET')
authorization_base_url = 'https://accounts.google.com/o/oauth2/auth'
token_url = 'https://oauth2.googleapis.com/token'
redirect_uri = 'http://localhost:5000/callback'

scopes = ['openid', 'https://www.googleapis.com/auth/userinfo.profile', 'https://www.googleapis.com/auth/userinfo.email']

oauth = OAuth2Session(client_id, redirect_uri=redirect_uri, scope=scopes)

@app.route("/")
def login():
    authorization_url, state = oauth.authorization_url(authorization_base_url, access_type="offline", prompt="consent")
    return redirect(authorization_url)

@app.route("/callback")
def callback():
    authorization_response = request.url
    token = oauth.fetch_token(token_url, authorization_response=authorization_response, client_secret=client_secret)

    response = oauth.get('https://www.googleapis.com/oauth2/v1/userinfo?alt=json')
    if response.status_code == 200:
        user_info = response.json()
        print(user_info)
        return f"Token récupéré avec succès ! <br> Info utilisateur : {user_info}, Token : {token}"
    else:
        return f"Erreur lors de la récupération des informations utilisateur: {response.status_code}"

if __name__ == "__main__":
    app.run(port=5000)
