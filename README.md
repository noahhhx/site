# site

My home website.

Using SpringBoot, Thymeleaf and made pretty with [WebTUI](https://webtui.ironclad.sh/start/intro/).

---

## Building

### Docker

Build and push to registry:
```shell
./mvnw compile jib:build
```
Build to local Docker daemon:
```shell
./mvnw compile jib:dockerBuild
```


## How it's deployed...

