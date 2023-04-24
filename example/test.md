# Test File

## Shell

Script in Markdown

```zsh
echo "nice in zsh"
echo "Arguments: ${*}"
```

```bash
echo "nice in bash"
echo "Arguments: ${*}"
```

## Python

And we can have Python too :)

```python
print("awesome")
```

some more python, spit up a script, so I can document easily.

```python
import sys

print(f"Arguments: {sys.argv}")
```

## Ruby

```ruby
puts "it works :)"
puts "Arguments: #{ARGV}"
```

## JSON config File

I can comment on sections
```json
{
```

Enable registration and disable debug mode:

```json
  "enable_registration": true,
  "debug": false,
```

Server information:

```json
  "hostname": "efertone.me",
  "port": 9999
```

And still get the whole JSON config file with export.
```json
}
```

### Lua

```lua
local variable = 15
print("Value:", variable)
```

### JavaScript with Node

```javascript
const fancy = function() {
  return "Fancy";
};
console.log(fancy());
```

### Custom tag

```something
const fancy = function() {
  return "Fancy";
};
console.log(fancy());
```
