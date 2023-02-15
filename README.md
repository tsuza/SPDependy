<div align="center">
  <h1><code>SPDependy</code></h1>
  <p>
    <strong>Quick way to install plugin dependencies ( includes ). It's mainly meant to be used with Github Actions, but it can be used normally just fine.</strong>
  </p>
  <p style="margin-bottom: 0.5ex;">
    <img
        src="https://img.shields.io/github/downloads/Zabaniya001/SPDependy/total"
    />
    <img
        src="https://img.shields.io/github/last-commit/Zabaniya001/SPDependy"
    />
    <img
        src="https://img.shields.io/github/issues/Zabaniya001/SPDependy"
    />
    <img
        src="https://img.shields.io/github/issues-closed/Zabaniya001/SPDependy"
    />
    <img
        src="https://img.shields.io/github/repo-size/Zabaniya001/SPDependy"
    />
    <img
        src="https://img.shields.io/github/workflow/status/Zabaniya001/SPDependy/Compile%20and%20release"
    />
  </p>
</div>

## Example ##
Here is one of my repositories that use this script to compile and release a plugin: [AllClassReviveMarker](https://github.com/Zabaniya001/AllClassReviveMarker)


## Requirements to compile the script  ##
1. Clone the repository.
2. Run `go mod download` inside the folder where the repository resides. This download the dependencies.
3. Run `go build`.
4. It's compiled!

### List of used third-party modules ###

- [Toml by BurntSushi](https://github.com/BurntSushi/toml)

## sp.toml ##
Note that the Compile section is not used by the Go script itself, but it gets used by the github action to compile all the listed plugins.

To add includes, you just need to add:
```
[[dependency]]
url = "github url to the include folder OR include file"
path = "directory where you want to download the includes. If you just want to download them inside the include folder, just put a ."
```

Here is an example:
```
[Compile]
plugins = [
    "AllClassReviveMarker",
    "test"
]

[[dependency]]
url = "https://github.com/nosoop/stocksoup"
path = "stocksoup"

[[dependency]]
url = "https://github.com/DosMike/tf2hudmsg/blob/master/tf2hudmsg.inc"
path = "."
```

## Usage in GitHub Actions ##
1. Create a `sp.toml` at the base of your repository where you list your dependencies ( [check the sp.toml in this repository if you want an example](sp.toml) ).
2. Copy the folder [.github/workflows that is in example](example) inside your own repository.
  - The github action will assume that your plugin has the same name as your repository. This is used ONLY for creating the tag for the release. If it has a different name, go to the .yml scripts and change `${{ github.event.repository.name }}` into your plugin's name ( for example `hello_plugin`, without the .sp ) from `echo "PLUGIN_NAME_ENV=${{ github.event.repository.name }}" >> $GITHUB_ENV`.
3. That's it.

**NB**: The Github Actions use linux and it's case-sensitive, so make sure to have all of your directories ( plugins, include, scripting, gamedata, translations ) lower case.

## Usage outside of GitHub Actions ##
1. Download the latest [executable](https://github.com/Zabaniya001/SPDependy/releases) and run it inside the folder where sp.toml resides.

## TO-DO ##
- Implement a Rust-like approach where you can just list the names of the include ( e.g. Stocksoup ) and it'll automatically fetch the URL.

## N.B ##
**This is my first Go script and I learnt it on the spot while writing this script. If there are any suggestions or improvements, open an issue!**