## Create Blog

```sh
POST /api/v1/blogs HTTP/1.1
Authorization: Bearer {{token}}
Content-Type: application/json; charset=utf-8
Host: miniblogapi.buildwithrs.dev
Connection: close
User-Agent: RapidAPI/4.4.3 (Macintosh; OS X/15.3.1) GCDHTTPRequest
Content-Length: 78

{"title":"Awesome Day","content":"\u4eca\u5929\u662f\u4e2a\u597d\u65e5\u5b50"}
```

-- response:

```sh
HTTP/1.1 200 OK
Date: Fri, 10 Oct 2025 05:38:08 GMT
Content-Type: application/json
Content-Length: 173
Connection: close
access-control-allow-origin: *
vary: origin, access-control-request-method, access-control-request-headers

{"blog":{"id":5,"title":"Awesome Day","content":"今天是个好日子","author_id":2,"created_at":"2025-10-10T05:38:08.305069Z","updated_at":"2025-10-10T05:38:08.305069Z"}}
```

## Update blog

-- request:

```sh
PUT /api/v1/blogs/5 HTTP/1.1
Authorization: Bearer {{token}}
Content-Type: application/json; charset=utf-8
Host: miniblogapi.buildwithrs.dev
Connection: close
User-Agent: RapidAPI/4.4.3 (Macintosh; OS X/15.3.1) GCDHTTPRequest
Content-Length: 85

{"title":"awesome day","content":"\u4eca\u5929\u662f\u4e2a\u597d\u65e5\u5b50","id":5}
```

-- response:

```sh
HTTP/1.1 200 OK
Date: Fri, 10 Oct 2025 05:46:16 GMT
Content-Type: application/json
Content-Length: 173
Connection: close
access-control-allow-origin: *
vary: origin, access-control-request-method, access-control-request-headers

{"blog":{"id":5,"title":"awesome day","content":"今天是个好日子","author_id":2,"created_at":"2025-10-10T05:38:08.305069Z","updated_at":"2025-10-10T05:46:16.740504Z"}}
```

## List Blogs

-- request:

```sh
GET /api/v1/blogs HTTP/1.1
Authorization: Bearer {{token}}
Host: miniblogapi.buildwithrs.dev
Connection: close
User-Agent: RapidAPI/4.4.3 (Macintosh; OS X/15.3.1) GCDHTTPRequest
```

- response:
```sh
{"blogs":[{"id":1,"title":"Learning Rust","author_id":1,"created_at":"2025-10-09T05:05:02.302422Z","updated_at":"2025-10-09T05:05:02.302422Z"},{"id":4,"title":"Learning Rust with me","author_id":1,"created_at":"2025-10-10T05:25:00.894443Z","updated_at":"2025-10-10T05:25:00.894443Z"},{"id":2,"title":"Learning Rust1","author_id":1,"created_at":"2025-10-09T05:25:12.385391Z","updated_at":"2025-10-09T05:25:12.385391Z"},{"id":3,"title":"Learning Rust1","author_id":1,"created_at":"2025-10-10T05:23:09.005547Z","updated_at":"2025-10-10T05:23:09.005547Z"}]}
```

## Delete blog

-- request:
```sh
DELETE /api/v1/blogs/6 HTTP/1.1
Authorization: Bearer {{token}}
Content-Type: application/json; charset=utf-8
Host: miniblogapi.buildwithrs.dev
Connection: close
Content-Length: 2

-- response:

HTTP/1.1 200 OK
Date: Fri, 10 Oct 2025 05:50:19 GMT
Content-Type: application/json
Content-Length: 51
Connection: close
access-control-allow-origin: *
vary: origin, access-control-request-method, access-control-request-headers
```