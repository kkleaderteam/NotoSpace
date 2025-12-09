Build docker image for Noto worker manually

```shell
docker buildx build -f ./services/Noto-worker/Dockerfile  --platform linux/amd64 -t Notoinc/Noto_worker --push .
```