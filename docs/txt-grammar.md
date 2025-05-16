# `.txt` grammar

## Grammar for basic usage

```
{tag_name} {attr_1} {value_1} | {attr_2} {value_2} | {attr_3} {value_3} > {children} <
```

- You should start with the name of the tag.
- You can add pairs of attributes and their value for the tag and you can add one single space(` `) right after the attribute to separate it from its value. If you want to add multiple attributes, you should add a pipe(`|`) and continue from there.
- You can close the tag with `/`, which will be parsed into ` />`(used for self-closing), or open the tag with `>` and close it with `<` later with children like above.

## Grammar for components definition/usage

### Definition

```
a href #url | target #target > span > #main_content < <
```

- Components should have their names starting with a single underscore(`_`), not to be confused with normal tags.
- You can add any content to the components.
- If you want to receive any arguments, then you can add `#{argument_name}` that gets passed from the one using this component.

### Usage

```
_better_a url https://github.com/iamseol/txtweb | target _blank | main_content click here to go to the txtweb github repository /
```

- Using components is exactly the same as using normal HTML tags but you should always close the tag with `/` and the parameters have to get passed.
