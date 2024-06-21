https://learn.microsoft.com/pt-br/rest/api/storageservices/authorize-with-shared-key
# Blob, Fila e Serviços de Arquivos (autorização de chave compartilhada)
```
StringToSign = VERB + "\n" +  
               Content-Encoding + "\n" +  
               Content-Language + "\n" +  
               Content-Length + "\n" +  
               Content-MD5 + "\n" +  
               Content-Type + "\n" +  
               Date + "\n" +  
               If-Modified-Since + "\n" +  
               If-Match + "\n" +  
               If-None-Match + "\n" +  
               If-Unmodified-Since + "\n" +  
               Range + "\n" +  
               CanonicalizedHeaders +   
               CanonicalizedResource;
```
e.g.
```
GET\n\n\n\n\n\n\n\n\n\n\n\nx-ms-date:Fri, 26 Jun 2015 23:39:12 GMT\nx-ms-version:2015-02-21\n/myaccount/mycontainer\ncomp:metadata\nrestype:container\ntimeout:20
```
or
```
GET\n /*HTTP Verb*/  
\n    /*Content-Encoding*/  
\n    /*Content-Language*/  
\n    /*Content-Length (empty string when zero)*/  
\n    /*Content-MD5*/  
\n    /*Content-Type*/  
\n    /*Date*/  
\n    /*If-Modified-Since */  
\n    /*If-Match*/  
\n    /*If-None-Match*/  
\n    /*If-Unmodified-Since*/  
\n    /*Range*/  
x-ms-date:Fri, 26 Jun 2015 23:39:12 GMT\nx-ms-version:2015-02-21\n    /*CanonicalizedHeaders*/  
/myaccount /mycontainer\ncomp:metadata\nrestype:container\ntimeout:20    /*CanonicalizedResource*/
```
```
Authorization="[SharedKey|SharedKeyLite] <AccountName>:<Signature>"
```