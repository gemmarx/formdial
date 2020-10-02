# Formdial
Formdial is a tool to display a markdown document with some input fields from command line.

<img src="https://user-images.githubusercontent.com/6276021/94700459-73dfc080-0376-11eb-97ec-46683e96a40c.png" width="350px">

### Dependencies
Markdown parser:  [markdown-it](https://github.com/markdown-it/markdown-it)  
DSL for html form:  [markdown-it-input](https://github.com/rajgoel/markdown-it-input)  
Rust binding of webview:  [web-view](https://github.com/Boscop/web-view)  
Style sheet:  [tufte-css](https://github.com/edwardtufte/tufte-css)

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

### Build
```bash
$ npm install
$ cargo build

# If you need to install
$ cargo install --path .
```

### Example
```bash
# Linux / Mac
$ formdial sample.md |jq
# or
$ formdial <sample.md |jq
```

```powershell
# MS-Windows
PS> formdial.exe sample.md |ConvertFrom-Json |ConvertTo-Json
```

### Input:
sample.md
```markdown
# Sample form

## My information
Name [id] = ___
Sex [sex] = () Male | 1 () Female | 2 () None | 9
Eye color [eye] = { Amber | amb ; Blue | blu ; Brown | bro ; Gray | gry ; Green | grn ; Hazel | haz }

### Check all that apply
[condition] = [] Over 1.80 m tall | tall [] Over 90 kg | heavy

### Describe your athletic ability
"""
"""[ability]
```

### Pop-up dialog

Linux
<img src="https://user-images.githubusercontent.com/6276021/94457458-8bd90800-01ef-11eb-96b1-0f47272d6744.png" width="600px">

MS-Windows
<img src="https://user-images.githubusercontent.com/6276021/94558007-5b9a7380-029a-11eb-8c8c-ac6738bdfdfa.png" width="450px">

Mac
<img src="https://user-images.githubusercontent.com/6276021/94578155-534e3280-02b2-11eb-9bfe-7c8313f32760.png" width="600px">

### Output:
```json
{
  "id": "Alice",
  "sex": "2",
  "condition-tall": false,
  "condition-heavy": true,
  "ability": "I’m good at dancing.\nMy tennis skill is high.",
  "eye": "grn"
}
```
