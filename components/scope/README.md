# `scope`

Selects keys in a wasi:config/store matching a scope prefix based on the content of a 'scope' import which mimics wasi:config/store. The scope value is removed from the resulting keys, and keys not matching the scope prefix are ignored.

Config values:
```yaml
greeting.casual: hey
greeting.formal: hello
greeting.hawaiian: aloha
not.a.greeting: is ignored
```

Scope values:
```yaml
prefix: greeting.
```

Resulting values:
```yaml
casual: hey
formal: hello
hawaiian: aloha
```
