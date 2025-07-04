# ClipWeb
The backend for [clip](https://github.com/Sushi-Mampfer/clip), feel free to use it for your own projects or tools.

## Api
### /create
**POST**

**Body:**
```json
{
    "content": "Your data",
    "expiry": "Time to expire in seconds(max 86400, 24h)"
}
```