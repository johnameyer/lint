* Empty lines indentation or lack thereof
* Cascading wraps
* Wrapping within annotations
* Annotations newline
* Newline between static imports and non-static imports
* Comments - RmsApiUtil.java
* No need for extra newline after block if followed by additional block close



Wrapping!!!!

Six cases

### A. No wrapping
Everything fits (intrinsic width < max length)
```
x.y(a, b, c)
```
### B. Wrap the parameters onto a single newline
Not everything fits. x and y fit on the same line as do a, b, c

```
x.y(
    a, b, c
)
```
### C. Wrap all the parameters on separate lines
Not everything fits. The parameters do not all fit on a single line.
```
x.y(
    a,
    b,
    c
)
```
### D. Wrap the parent
Not everything fits. x.y does not fit on the same line, but y still fits with a, b, c
```
x
    .y(a, b, c)
```
### E. Wrap the parent and children
Not everything fits. x.y does not fit on the same line, and while a,b,c all do, .y a,b,c does not
```
x
    .y(
        a, b, c
    )
```
### F. Wrap the parent and children
Not everything fits. x.y does not fit on the same line, and neither does a,b,c.
```
x
    .y(
        a,
        b,
        c
    )
```

### Hypothesis

First, do a, b, c fit on the same line? No? Then they have to be indented.

Otherwise, do x, y fit on the same line? If so, they're fine

If not, does a,b,c fit with x / x.y? Then 

1. Recurse into children. Wrap if needed

Now, we know that a, b, c fits on a single line
2. Check length of remaining content i.e. x.y(a,b,c), with wrap_parent false or similar

3. 


[x.y(][[a,][b,][c]][)]


### A. No wrapping


### B. Cascade
Not everything fits. x.a fits on a single line.
```
x.a()
    .b()
    .c()
    .d()
```

### C. Supercascade
Not everything fits. x.a does not fit on a single line.
```
x
    .a()
    .b()
    .c()
    .d()
```

### D. With Params
Not everything fits.
```
u.a(x, y, z)
    .b()
    .c()
    .d()
```
### 
```
u
    .a(
       x, y, z
    )
    .b()
    .c()
    .d()
```
or 
```
u.a(
    x, y, z
)
    .b()
    .c()
    .d()
```
### 
```
u.a()
    .b(x, y, z)
    .c()
    .d()
```
### 
```
u.a()
    .b(x, y, z)
    .c()
    .d()
```


* wrap_parameters shows the current issue of preferring to wrap after the dot rather than wrapping the children
    * This was fixed using stack poppers / pushers to create an additional level of nesting - but this might cause other issues itself
* for_loop illustrates that the for loops do not currently have any wrapping enabled
    * Fixed


* unwrappable_child test illustrates an issue. Despite the child being wrapped, it still violates the wrapping of the parent
* force_children test illustrates an issue. Despite the parent being wrapped, the other method calls within are not wrapped.
    * We need to propogate the wrapping down the chain -> parent_wrap_causes_wrap
    * Or is prevent_wrap_cascade meant to do this???
* If content illustrates a contrary case to unwrappable child - max line is violated on elements that do not have wrappable children
* Try_with_resources


* large files illustrate an issue. Due to needing to first render the content, and then render a second time if wrapping, it takes a long time to run
* If there is a newline, can a loseless mode be added to prevent overriding the user's manual wrap even if length is not long enough?