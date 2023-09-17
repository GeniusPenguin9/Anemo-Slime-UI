Anemo Slime Server API
======================

GET: /static/*
--------------
Static assets, html, css, js, etc

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
    "widgetsData": [
        {
            "widgetId": "wwwwwwww",
            "property": "text",
            "value": "hello world"
        }
    ]
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
    "widgetsData": [
        {
            "widgetId": "wwwwwwww",
            "property": "text",
            "value": "hello world"
        }
    ]
}
```
