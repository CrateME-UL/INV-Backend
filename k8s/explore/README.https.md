### to generate a self signed certificate with private key using mkcert (for testing only):
```bash
sudo apt install mkcert
mkcert monequilibre.inventaire.local
```

### make sure to add the host to "C:\Windows\System32\drivers\etc\hosts" on Windows and link the ip address of the wsl to the hostname at the end of the file

```bash
w.x.y.z monequilibre.inventaire.local
```

### apply the webserver-* files in the k8s cluster with

```bash
sudo k3s kubectl apply -f .
```

### on the browser to access the site type:
```bash
[monequilibre.inventaire.local](http://monequilibre.inventaire.local/)
```

### create the certificate and add it to the cluster
```bash
sudo k3s kubectl create secret tls mkcert-tls-secret \
  --cert=monequilibre.inventaire.local.pem \
  --key=monequilibre.inventaire.local-key.pem
```

### on the browser to access the site type (using self signed https):
```bash
[monequilibre.inventaire.local](https://monequilibre.inventaire.local/)
```

### note: you can also access it through the http url, you will be redirected towards https

