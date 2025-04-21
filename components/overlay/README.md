# `overlay`

Combines value from two wasi:config/stores into a single store. The 'store' import is the base set of values, while the 'overlay' import, a mirror of the store interface, contributes additional values overriding in the case of a conflict.

Config values:
```yaml
greeting: hello
bow: "false"
```

Overlay values:
```yaml
greeting: aloha
lei: "true"
```

Resulting values:
```yaml
greeting: aloha
bow: "false"
lei: "true"
```
