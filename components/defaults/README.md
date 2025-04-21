# `defaults`

Combines value from two wasi:config/stores into a single store. The 'store' import is the base set of values, while the 'defaults' import, a mirror of the store interface, contributes default values when the config does not contain a value.

Config values:
```yaml
greeting: aloha
lei: "true"
```

Default values:
```yaml
greeting: hello
bow: "false"
```

Resulting values:
```yaml
greeting: aloha
bow: "false"
lei: "true"
```
