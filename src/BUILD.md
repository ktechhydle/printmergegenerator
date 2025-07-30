## Build And Deploy PrintMergeGenerator
_Make sure Dioxus and Dioxus-CLI are installed on your system!_

1. Clear the `docs` directory in the root of this repository:

```
rm docs/*

# on bash/unix systems
rm -rf docs/* docs/.*
```

2. Bundle the app with `dx` and move the public folders content's into `docs`:

```
dx bundle --out-dir docs
mv docs/public/* docs
```

3. Clone the `index.html` and name it `404.html` (required so that PrintMergeGenerator will work with client-side routing):

```
cp docs/index.html docs/404.html
```

## Final Script

```shell
rm docs/*
dx bundle --out-dir docs
mv docs/public/* docs
cp docs/index.html docs/404.html
rm docs/public
```
