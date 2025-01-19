## Codespaces or classic Gitpod

Aside from installing cargo and running it, 

you can just use the native docker.

```sh
docker build -t rustweb .
docker run -p 8080:8080 --rm -ti rustweb
```
