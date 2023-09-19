Anemo Slime Server API
======================

GET: /
--------------
return static index.html

GET: /favicon.ico
--------------
return static favicon.ico

GET: /assets/{fileName}
--------------
map to static assets folder

POST: /api/view/{viewName}
--------------
Create a view and viewmodel session, or attach a view to existing viewmodel session

Request:
* Path params: 
    * viewName: the name of the view to display
* Query params: None
* Body: 
```
{
  // [optional] if set, attaching a view to existing viewmodel (usually find in popup dialog); 
  // if not set, server will create a view and viewmodel session for this view request
  "viewmodelId": "xxxxx" 
}
```

Response:
* Body: a json that contains viewmodelId and widgets data
```
{
    "viewmodelId": "xxxxxxxx", 
    "widgetsData": {
        // key is widget id
        "wwwwwwww": {
            "text": "hello",
            "visible": true
        },
        "yyyyyyyy": {
            "text": "morning",
            "select": false
        }
    }
}
```

POST: /api/action
-----------------
Create a view and viewmodel session, or attach a view to existing viewmodel session

Request:
* Path params: None
* Query params: None
* Body: 
```
{
  "viewmodelId": "xxxxx",
  "widgetId": "wwwww",
  "actionType": "change", // e.g. when a textbox value changes
  "data": {
    "text": "vvvvvv"
  } 
}
```

Response:
* Body: a json that contains widgets data
```
{
    "viewmodelId": "xxxxxxxx", 
    "widgetsData": {
        // key is widget id
        "wwwwwwww": {
            "text": "hello",
            "visible": true
        },
        "yyyyyyyy": {
            "text": "morning",
            "select": false
        }
    }
}
```
