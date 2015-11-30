import requests
import json

url = 'http://127.0.0.1:6767'
payload = {'val': 'गरम'}
headers = {'content-type': 'application/json'}

r = requests.post(url=url, data=json.dumps(payload), headers=headers)
print(r.content.decode('utf-8', 'strict'))
print(r.json)
print(type(r.json))
#print(dir(r))
