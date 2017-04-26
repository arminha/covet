Scan Status
-----------

### Request

GET /Scan/Status

No authentication required

### Response

#### Headers

```
HTTP/1.1 200 OK
Server: HP HTTP Server; HP ENVY 7640 series - [..]
Content-Type: text/xml
Transfer-Encoding: chunked
Content-Encoding: gzip
Cache-Control: must-revalidate, max-age=0
Pragma: no-cache
```

#### Payload

```xml
<?xml version="1.0" encoding="UTF-8"?>
<ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
	<ScannerState>Idle</ScannerState>
	<AdfState>Empty</AdfState>
</ScanStatus>
```
or
```xml
<?xml version="1.0" encoding="UTF-8"?>
<ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
	<ScannerState>BusyWithScanJob</ScannerState>
	<AdfState>Empty</AdfState>
</ScanStatus>
```
or
```xml
<?xml version="1.0" encoding="UTF-8"?>
<ScanStatus xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
	<ScannerState>Idle</ScannerState>
	<AdfState>Loaded</AdfState>
</ScanStatus>
```
