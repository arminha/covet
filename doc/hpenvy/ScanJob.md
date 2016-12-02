Scan Job
--------

### Request

#### Headers

```
POST /Scan/Jobs HTTP/1.1
Content-Type: text/xml
Content-Length: 913
Cookie: sid=se9d34605-7045f268cb76f8769b78f916701b256d
```

Uses basic authentication

#### Payloads

JPEG scan from glass with 300dpi.

```
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fw="http://www.hp.com/schemas/imaging/con/firewall/2011/01/05">
  <scan:XResolution>300</scan:XResolution>
  <scan:YResolution>300</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Jpeg</scan:Format>
  <scan:CompressionQFactor>25</scan:CompressionQFactor>
  <scan:ColorSpace>Color</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Platen</scan:InputSource>
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Photo</scan:ContentType>
</scan:ScanJob>
```

PDF scan from glass with 300dpi.

```
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19"
		xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
		xmlns:fw="http://www.hp.com/schemas/imaging/con/firewall/2011/01/05">
	<scan:XResolution>300</scan:XResolution>
	<scan:YResolution>300</scan:YResolution>
	<scan:XStart>0</scan:XStart>
	<scan:YStart>0</scan:YStart>
	<scan:Width>2480</scan:Width>
	<scan:Height>3508</scan:Height>
	<scan:Format>Pdf</scan:Format>
	<scan:CompressionQFactor>25</scan:CompressionQFactor>
	<scan:ColorSpace>Color</scan:ColorSpace>
	<scan:BitDepth>8</scan:BitDepth>
	<scan:InputSource>Platen</scan:InputSource>
	<scan:GrayRendering>NTSC</scan:GrayRendering>
	<scan:ToneMap>
		<scan:Gamma>1000</scan:Gamma>
		<scan:Brightness>1000</scan:Brightness>
		<scan:Contrast>1000</scan:Contrast>
		<scan:Highlite>179</scan:Highlite>
		<scan:Shadow>25</scan:Shadow>
	</scan:ToneMap>
	<scan:ContentType>Document</scan:ContentType>
</scan:ScanJob>
```

PDF scan from glass with 600dpi.

```
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fw="http://www.hp.com/schemas/imaging/con/firewall/2011/01/05">
  <scan:XResolution>600</scan:XResolution>
  <scan:YResolution>600</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Pdf</scan:Format>
  <scan:CompressionQFactor>25</scan:CompressionQFactor>
  <scan:ColorSpace>Color</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Platen</scan:InputSource>
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Document</scan:ContentType>
</scan:ScanJob>
```

PDF scan from ADF with 600dpi.

```
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fw="http://www.hp.com/schemas/imaging/con/firewall/2011/01/05">
  <scan:XResolution>600</scan:XResolution>
  <scan:YResolution>600</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Pdf</scan:Format>
  <scan:CompressionQFactor>25</scan:CompressionQFactor>
  <scan:ColorSpace>Color</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Adf</scan:InputSource>
  <scan:AdfOptions/>
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Document</scan:ContentType>
</scan:ScanJob>
```

PDF scan from ADF with 300dpi grayscale.

```
<scan:ScanJob xmlns:scan="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fw="http://www.hp.com/schemas/imaging/con/firewall/2011/01/05">
  <scan:XResolution>300</scan:XResolution>
  <scan:YResolution>300</scan:YResolution>
  <scan:XStart>0</scan:XStart>
  <scan:YStart>0</scan:YStart>
  <scan:Width>2480</scan:Width>
  <scan:Height>3508</scan:Height>
  <scan:Format>Pdf</scan:Format>
  <scan:CompressionQFactor>25</scan:CompressionQFactor>
  <scan:ColorSpace>Gray</scan:ColorSpace>
  <scan:BitDepth>8</scan:BitDepth>
  <scan:InputSource>Adf</scan:InputSource>
  <scan:AdfOptions/>
  <scan:GrayRendering>NTSC</scan:GrayRendering>
  <scan:ToneMap>
    <scan:Gamma>1000</scan:Gamma>
    <scan:Brightness>1000</scan:Brightness>
    <scan:Contrast>1000</scan:Contrast>
    <scan:Highlite>179</scan:Highlite>
    <scan:Shadow>25</scan:Shadow>
  </scan:ToneMap>
  <scan:ContentType>Document</scan:ContentType>
</scan:ScanJob>
```

### Response

```
HTTP/1.1 201 Created
Location: http://192.168.1.2:443/Jobs/JobList/2
Content-Length: 0
Cache-Control: must-revalidate, max-age=0
Pragma: no-cache
```
