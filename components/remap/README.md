# `remap`

Maps the keys in a wasi:config/store based on the content of a 'mapping' import which mimics wasi:config/store.

Config values:
```yaml
greeting: hello
hawaiian-greeting: aloha
```

Mapping values:
```yaml
greeting: hawaiian-greeting
```

Resulting values:
```yaml
greeting: aloha
hawaiian-greeting: aloha
```
