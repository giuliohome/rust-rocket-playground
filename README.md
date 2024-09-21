## Containerization

Use a Dockerfile from [rocket guide](https://rocket.rs/guide/v0.5/deploying/#containerization), but **modified** to include a custom root certificate (copy it here!) and a json folder.

- start your Linux from this root folder using a cool [windows terminal](https://apps.microsoft.com/detail/9n0dx20hk701?hl=en-us&gl=IT)
```sh
wsl
```
- build with Kaniko and export to a local tar
```sh
sudo ctr run --net-host --rm --mount type=bind,src=$(pwd),dst=/workspace,options=rbind:rw gcr.io/kaniko-project/executor:latest kaniko-executor /kaniko/executor --dockerfile=/workspace/Dockerfile --context=/workspace --no-push --skip-tls-verify --build-arg pkg=hello-rocket --tarPath=/workspace/rust-playground-kaniko.tar --destination=localhost/rust-playground-kaniko:latest
```

- Import the image into containerd
```sh
sudo ctr image import rust-playground-kaniko.tar
```

- Create the container
```sh
sudo ctr c create --net-host localhost/rust-playground-kaniko:latest rust-playground
```

- Start the task
```sh
sudo ctr t start rust-playground
```

- Open in the browser
  - http://localhost:8080/
  - http://localhost:8080/json
  - http://localhost:8080/api/data

- Eventually Split the terminal window and stop the task
```sh
sudo ctr t kill -s 9 rust-playground
```
