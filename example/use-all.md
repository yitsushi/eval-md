# Use all code blocks

This is a weird case, but someone said they would use it and that would be cool.

## Imports

```python
from string import Template
```

## Constants

```python
TIMEOUT: int = 5
```

### HTML Template

```python
TEMPLATE: str = '''
```

```html
<html>
  <head><title>Nice</title></head>
  <body>
    <div id="app">${body}</div>
  </body>
</html>
```

```python
'''
```

## Main

```python
print('Do something with:')
template = Template(TEMPLATE)
print(template.safe_substitute(body="awesome"))
```
