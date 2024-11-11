### to generate a self signed certificate with private key using mkcert (for testing only):
```bash
sudo apt install mkcert
mkcert monequilibre.inventaire.local
```

### make sure to add the host to "C:\Windows\System32\drivers\etc\hosts" and link the ip address of the wsl to the hostname at the end of the file

```bash
x.y.z monequilibre.inventaire.local
```

### apply the webserver-* files in the k8s cluster with

```bash
sudo k3s kubectl apply -f k8s/explore
```
