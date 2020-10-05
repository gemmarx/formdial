# Formdial
Formdial is a tool to pop-up a markdown document from command line with some input fields.

<img src="https://user-images.githubusercontent.com/6276021/94700459-73dfc080-0376-11eb-97ec-46683e96a40c.png" width="350px">

### Dependencies
Markdown parser:  [markdown-it](https://github.com/markdown-it/markdown-it)  
DSL for html form:  [markdown-it-input](https://github.com/rajgoel/markdown-it-input)  
Rust binding of webview:  [web-view](https://github.com/Boscop/web-view)  
Style sheet:  [tufte-css](https://github.com/edwardtufte/tufte-css)

### Build
```bash
$ npm install
$ cargo build

# to install
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
<img src="https://user-images.githubusercontent.com/6276021/95030920-d1b13700-06ed-11eb-9658-a7d61f266ca0.png" width="480px">

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

### Another input style
Put down html tags directly
if you need controls such as default value, placeholder, validation, etc.

sample2.md
```markdown
# Sample form

## My information
<div>
Name
<input placeholder="my name" type="text" name="id">
</div>

<div>
Sex
<label><input checked=true type="radio" name="sex" value="1">Male</label>
<label><input type="radio" name="sex" value="2">Female</label>
<label><input type="radio" name="sex" value="9">None</label>
</div>

<div>
Eye color
<select name="eye">
<option value="amb">Amber</option>
<option value="blu">Blue</option>
<option value="bro">Brown</option>
<option value="gry">Gray</option>
<option value="grn">Green</option>
<option value="haz">Hazel</option>
</select>
</div>

### Check all that apply
<div>
<label><input type="checkbox" name="condition" value="tall">Over 1.80 m tall</label>
<label><input type="checkbox" name="condition" value="heavy">Over 90 kg</label>
</div>

### Describe your athletic ability
<div>
<textarea required name="ability">
</textarea>
</div>
```

<img src="https://user-images.githubusercontent.com/6276021/95011048-e39cc680-0668-11eb-8a9f-ddc645290498.png" width="480px">

### Other options
```
An HTML form generator for shell scripts.

USAGE:
    formdial [OPTIONS] [--] [INPUT]

ARGS:
    <INPUT>    input file (markdown)

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --background <background>    background image
        --css <css>...               style sheet
        --height <height>            size param (pixel) [default: 480]
        --button-submit <submit>     label of submit button [default: submit]
        --title <title>              window title [default: Input Form]
        --width <width>              size param (pixel) [default: 680]
```
