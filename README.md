# Web-Request-Logger

Listens to "127.0.0.1:7878" adding all http Request to log.txt:

    - Timestamp
    - Sender ip
    - Http start line
    - Headers
    - Currently ignores body (not the content length header though)

Responds to all request with "200 OK".

A request might loog like this:

    curl "http://127.0.0.1:7878" \
        -X PATCH \
        -d "ABC" \
        -H "Connection: keep-alive" \
        -H "Accept-Language: en-GB, en;q=0.5" 

The corresponding log entry should look like this:

        -------------------------------------------
        Received Request at:    09/10/2023 17:49:32
        From IP:	    	    127.0.0.1:7878
        Request:
        POST / HTTP/1.1
        Host: 127.0.0.1:7878
        Connection: keep-alive
        Accept-Language: en-GB, en;q=0.5
        content-length: 3

Next steps include:

    Parseing the Request into structs to allow easy passing onto methods
    Saving logging information to a Database
    
