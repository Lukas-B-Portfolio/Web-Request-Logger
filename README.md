# Web-Request-Logger

Expects postgres database at localhost:5432/HttpRequestLog with Username/Password rust/rust.

Listens to 127.0.0.1:7878 and logs Http requests to the database:

    - Timestamp
    - Sender ip
    - Http start line (verb, target, version)
    - Headers
    - Plain text bodies (ignores files, media etc.)

Responds to all requests with "200 OK".

A request might look like this:

    curl "http://127.0.0.1:7878" \
        -X PATCH \
        -d "ABC" \
        -H "Connection: keep-alive" \
        -H "Accept-Language: en-GB, en;q=0.5" 
