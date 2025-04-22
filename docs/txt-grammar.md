# `.txt` grammar

## Grammar for basic usage

Before using this tool, you have to know the grammar that it can understand. As the name serves, `.txt` is the only file extension it can parse. Below is the basic format and rules.

```
{tag_name} {attr_1} {value_1} | {attr_2} {value_2} | {attr_3} {value_3} > {children} <
```

1. You should start with the name of the tag.
2. You can add the pair of attributes and their value for the tag and you can add one single space(` `) right after where the attribute is to separate it from its value. If you want to add multiple attributes, you should add a pipe(`|`) and continue from there.
3. You should end it with `><` to close the tag. `><` will be parsed into ` />` which is used for self-closing. If you want to add more content inside of the tag, then you can add it between `>` and `<`. However, you cannot add a paragraph of text with tags in it.

## Grammar for components definition

To define a component, all you need to do is to add contents that the component should have but with `#{number}`, where the number is the order in which the value comes in.

If there is a component defined at `/components/_better_a.txt`,

```
a href #1 | target #2 > span > #3 < <
```

You can use it by adding.

```
_better_a https://github.com/iamseol/txtweb | _blank | click here to go to the txtweb github repository >
```

And, it will be parsed like the below.

```
<a href="https://github.com/iamseol/txtweb" target="_blank"><span>click to go to the txtweb github repository</span></a>
```

## Grammar for components usage

The grammar for components is very similar to the one for basic, except that you do not add pairs, but just values only.

```
{component_name} {value_1} | {value_2} | {value_3} >
```

1. You should start with the name of the components. The names should always start with `_`.
2. You should add the values matching up with the components' definitions. Every time you add more values, you should add pipes(`|`) between them to separate each other.
3. You should close the tag with `>`(not `><`).

## Indentation

The indentation does not matter at all.
