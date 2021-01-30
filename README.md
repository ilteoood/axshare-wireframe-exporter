# axshare-wireframe-exporter
Export Axshare's wireframe to PDF

# Usage

There are 3 parameters:
- baseUrl: url of the wireframe to be exported
- pdfName: name of the PDF to be generated
- openPopUp: will open the sticky notes inside the wireframe

# Execution example

```sh
> axshare-wireframe-exporter https://example.axshare.com/ example-pdf-export
Taking screenshot 0 for page https://example.axshare.com/test.html
Taking screenshot 1 for page https://example.axshare.com/test1.html
Taking screenshot 2 for page https://example.axshare.com/test2.html
Creating pdf example-pdf-export
```

# Installation
```sh
npm install -g axshare-wireframe-exporter
```

If you face an error like:
```sh
gyp WARN EACCES current user ("nobody") does not have permission to access the dev dir "/root/.cache/node-gyp/10.19.0"
gyp WARN EACCES attempting to reinstall using temporary dev dir "/usr/local/lib/node_modules/axshare-wireframe-exporter/node_modules/hummus/.node-gyp"
```

Consider to add the option ```npm_config_user=root``` as follow:
```sh
npm_config_user=root npm install -g axshare-wireframe-exporter
```