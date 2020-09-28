# formdial

### Using
Markdown parser:  [markdown-it](https://github.com/markdown-it/markdown-it)  
DSL for Html form:  [markdown-it-input](https://github.com/rajgoel/markdown-it-input)  
Rust Binding of webview:  [web-view](https://github.com/Boscop/web-view)  
CSS:  [tufte-css](https://github.com/edwardtufte/tufte-css)

```
An HTML form generator for shell scripting.

USAGE:
    formdial [OPTIONS] [--] [INPUT]

ARGS:
    <INPUT>    input file (markdown)

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --css <css>...       style sheet
        --height <height>    size param (pixel) [default: 600]
    -t, --title <title>      window title [default: Input Form]
        --width <width>      size param (pixel) [default: 850]
```

### Example
```bash
$ formdial sample.md |jq
# or
$ formdial <sample.md |jq
```

Input:  sample.md
```markdown
# Sample form

## My information
Name [name] = ___
Sex [sex] = () Male | male () Female | female () None  | none
Eye color [eye] = { Amber | amber ; Blue | blue ; Brown | brown ; Gray | gray ; Green | green ; Hazel | hazel }

### Check all that apply
[condition] = [] Over 6 feet tall | tall [] Over 200 pounds | heavy

### Describe your athletic ability
"""
"""[ability]
```

Pop-up dialog
<img src="https://user-images.githubusercontent.com/6276021/94457458-8bd90800-01ef-11eb-96b1-0f47272d6744.png" width="500px">

Output:
```json
{
  "name": "alice",
  "sex": "female",
  "condition-tall": false,
  "condition-heavy": true,
  "ability": "I’m good at dancing.\nMy tennis skill is high.",
  "eye": "green"
}
```
