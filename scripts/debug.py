import requests
import json

url = 'http://127.0.0.1:6767'
payload = {'val': 'गरम'}
headers = {'content-type': 'application/json'}

r = requests.post(url=url, data=json.dumps(payload), headers=headers)

s = r.content.decode('utf-8', 'strict')
obj = json.loads(s)
print(obj, type(obj))
print(obj['results'])

