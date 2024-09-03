
## Build a Webpage Using Blob Compilation

Blob compilation is a compiler option that tells the assembler to just emit simple fragments directly as output without any special formatting.
To create a blob webpage we can use the `LIB/default.html` library to help us get started.

```
import LIB/default.html;

(html(
   (head(
      (title 'Hello\sWorld_s)
   ))
   (body(
      (h2( 'Hello_s ))
   ))
))
```
