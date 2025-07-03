# ClipWeb
The backend for [clip](https://github.com/Sushi-Mampfer/clip), feel free to use it for your own projects or tools.

## Api
### /api/create
**POST**

**Body:**
```json
{
    data: "Your data",
    expiry: "Time to expire in minutes(max 1440)"
}
```