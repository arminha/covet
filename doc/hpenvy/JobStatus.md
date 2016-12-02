Scan Job Status
---------------

### Request

```
GET /Jobs/JobList/2 HTTP/1.1
Accept: application/xml, text/xml, */*
Cookie: sid=se9d34605-7045f268cb76f8769b78f916701b256d
```

### Response

#### Headers

```
HTTP/1.1 200 OK
Content-Type: text/xml
Etag: "42-6"
Cache-Control: must-revalidate, max-age=0
Pragma: no-cache
```

#### Payloads

First

```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/2</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-6</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PreScanPage>
      <PageNumber>1</PageNumber>
      <PageState>PreparingScan</PageState>
      <BufferInfo>
        <ScanSettings>
          <XResolution>300</XResolution>
          <YResolution>300</YResolution>
          <XStart>0</XStart>
          <YStart>0</YStart>
          <Width>2480</Width>
          <Height>3508</Height>
          <Format>Jpeg</Format>
          <CompressionQFactor>25</CompressionQFactor>
          <ColorSpace>Color</ColorSpace>
          <BitDepth>8</BitDepth>
          <InputSource>Platen</InputSource>
          <ContentType>Photo</ContentType>
        </ScanSettings>
        <ImageWidth>2480</ImageWidth>
        <ImageHeight>3508</ImageHeight>
        <BytesPerLine>7440</BytesPerLine>
        <Cooked>enabled</Cooked>
      </BufferInfo>
      <BinaryURL>/Scan/Jobs/2/Pages/1</BinaryURL>
      <ImageOrientation>Normal</ImageOrientation>
    </PreScanPage>
  </ScanJob>
</j:Job>
```
Later
```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/2</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-6</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PreScanPage>
      <PageNumber>1</PageNumber>
      <PageState>ReadyToUpload</PageState>
      <BufferInfo>
        <ScanSettings>
          <XResolution>300</XResolution>
          <YResolution>300</YResolution>
          <XStart>0</XStart>
          <YStart>0</YStart>
          <Width>2480</Width>
          <Height>3508</Height>
          <Format>Jpeg</Format>
          <CompressionQFactor>25</CompressionQFactor>
          <ColorSpace>Color</ColorSpace>
          <BitDepth>8</BitDepth>
          <InputSource>Platen</InputSource>
          <ContentType>Photo</ContentType>
        </ScanSettings>
        <ImageWidth>2480</ImageWidth>
        <ImageHeight>3508</ImageHeight>
        <BytesPerLine>7440</BytesPerLine>
        <Cooked>enabled</Cooked>
      </BufferInfo>
      <BinaryURL>/Scan/Jobs/2/Pages/1</BinaryURL>
      <ImageOrientation>Normal</ImageOrientation>
    </PreScanPage>
  </ScanJob>
</j:Job>
```
Almost done
```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/2</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-6</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PostScanPage>
      <PageNumber>1</PageNumber>
      <PageState>UploadCompleted</PageState>
      <TotalLines>3508</TotalLines>
    </PostScanPage>
  </ScanJob>
</j:Job>
```
Done
```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/2</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Completed</j:JobState>
  <j:JobStateUpdate>42-7</j:JobStateUpdate>
  <j:JobSource>userIO</j:JobSource>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PostScanPage>
      <PageNumber>1</PageNumber>
      <PageState>UploadCompleted</PageState>
      <TotalLines>3508</TotalLines>
    </PostScanPage>
  </ScanJob>
</j:Job>
```

##### Multiple pages

```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/6</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-22</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PreScanPage>
      <PageNumber>1</PageNumber>
      <PageState>PreparingScan</PageState>
      <BufferInfo>
        <ScanSettings>
          <XResolution>300</XResolution>
          <YResolution>300</YResolution>
          <XStart>0</XStart>
          <YStart>0</YStart>
          <Width>2480</Width>
          <Height>3508</Height>
          <Format>Pdf</Format>
          <CompressionQFactor>25</CompressionQFactor>
          <ColorSpace>Gray</ColorSpace>
          <BitDepth>8</BitDepth>
          <InputSource>Adf</InputSource>
          <ContentType>Document</ContentType>
        </ScanSettings>
        <ImageWidth>2480</ImageWidth>
        <ImageHeight>3508</ImageHeight>
        <BytesPerLine>2480</BytesPerLine>
        <Cooked>enabled</Cooked>
      </BufferInfo>
      <BinaryURL>/Scan/Jobs/6/Pages/1</BinaryURL>
      <ImageOrientation>Normal</ImageOrientation>
    </PreScanPage>
  </ScanJob>
</j:Job>
```

```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/6</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-22</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PreScanPage>
      <PageNumber>1</PageNumber>
      <PageState>PreparingScan</PageState>
      <BufferInfo>
        <ScanSettings>
          <XResolution>300</XResolution>
          <YResolution>300</YResolution>
          <XStart>0</XStart>
          <YStart>0</YStart>
          <Width>2480</Width>
          <Height>3508</Height>
          <Format>Pdf</Format>
          <CompressionQFactor>25</CompressionQFactor>
          <ColorSpace>Gray</ColorSpace>
          <BitDepth>8</BitDepth>
          <InputSource>Adf</InputSource>
          <ContentType>Document</ContentType>
        </ScanSettings>
        <ImageWidth>2480</ImageWidth>
        <ImageHeight>3508</ImageHeight>
        <BytesPerLine>2480</BytesPerLine>
        <Cooked>enabled</Cooked>
      </BufferInfo>
      <BinaryURL>/Scan/Jobs/6/Pages/1</BinaryURL>
      <ImageOrientation>Normal</ImageOrientation>
    </PreScanPage>
  </ScanJob>
</j:Job>
```

```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/6</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Processing</j:JobState>
  <j:JobStateUpdate>42-22</j:JobStateUpdate>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PostScanPage>
      <PageNumber>1</PageNumber>
      <PageState>UploadCompleted</PageState>
      <TotalLines>3508</TotalLines>
    </PostScanPage>

    <PreScanPage>
      <PageNumber>2</PageNumber>
      <PageState>PreparingScan</PageState>
      <BufferInfo>
        <ScanSettings>
          <XResolution>300</XResolution>
          <YResolution>300</YResolution>
          <XStart>0</XStart>
          <YStart>0</YStart>
          <Width>2480</Width>
          <Height>3508</Height>
          <Format>Pdf</Format>
          <CompressionQFactor>25</CompressionQFactor>
          <ColorSpace>Gray</ColorSpace>
          <BitDepth>8</BitDepth>
          <InputSource>Adf</InputSource>
          <ContentType>Document</ContentType>
        </ScanSettings>
        <ImageWidth>2480</ImageWidth>
        <ImageHeight>3508</ImageHeight>
        <BytesPerLine>2480</BytesPerLine>
        <Cooked>enabled</Cooked>
      </BufferInfo>
      <BinaryURL>/Scan/Jobs/6/Pages/2</BinaryURL>
      <ImageOrientation>Normal</ImageOrientation>
    </PreScanPage>
  </ScanJob>
</j:Job>
```

```
<?xml version="1.0" encoding="UTF-8"?>
<j:Job xmlns:j="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30"
    xmlns:dd="http://www.hp.com/schemas/imaging/con/dictionaries/1.0/"
    xmlns:fax="http://www.hp.com/schemas/imaging/con/fax/2008/06/13"
    xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
    xsi:schemaLocation="http://www.hp.com/schemas/imaging/con/ledm/jobs/2009/04/30 ../schemas/Jobs.xsd">
  <j:JobUrl>/Jobs/JobList/6</j:JobUrl>
  <j:JobCategory>Scan</j:JobCategory>
  <j:JobState>Completed</j:JobState>
  <j:JobStateUpdate>42-23</j:JobStateUpdate>
  <j:JobSource>userIO</j:JobSource>
  <ScanJob xmlns="http://www.hp.com/schemas/imaging/con/cnx/scan/2008/08/19">
    <PostScanPage>
      <PageNumber>2</PageNumber>
      <PageState>UploadCompleted</PageState>
      <TotalLines>3501</TotalLines>
    </PostScanPage>
  </ScanJob>
</j:Job>
```
