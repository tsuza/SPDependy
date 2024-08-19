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
Here is one of my repositories that uses this script to compile and release a plugin: [AllClassReviveMarker](https://github.com/tsuza/AllClassReviveMarker)


## Requirements to compile the script  ##
1. Clone the repository.
2. Run `rust build` ( if you don't have Rust, download it from the official website ).
4. It's compiled!

## sp.toml ##
Note that the Compile section is not used by the script itself, but it gets used by the github action to compile all the listed plugins.

To add includes, you just need to add:
```toml
[[dependency]]
url = "github url to the include folder OR include file"
path = "directory where you want to download the includes. If you just want to download them inside the include folder, just put a ."
```

Here is an example:
```toml
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
Just shove this in your workflow and you're good to go.
```yaml
- name: Download the includes downloader
  uses: tsuza/SPDependy@v2
  with:
    github_token: '${{ secrets.GITHUB_TOKEN }}'
```

If it's giving you any problems use v1 instead ( tsuza/SPDependy@v1 ) and open an issue.

### Variables: ###
```
github_token
  - Required.

file_name
  - Not required.
  - Default value: 'sp.toml'

output_directory
  - Not required.
  - Default value: '.temp/include/'
```

If you want an example workflow file, check out the [example](example) folder.

**NB**: Since this will be run on a Linux runner most likely, make sure to have all of your directories ( plugins, include, scripting, gamedata, translations ) lower case.

## Usage outside of GitHub Actions ##
1. Download the latest [executable](https://github.com/Zabaniya001/SPDependy/releases) and run it inside the folder where sp.toml resides.
  - *It's recommended to run it inside the terminal and put your [github token](https://github.com/settings/tokens/new) after it so you don't run against call request limits. However, you'll be just fine unless you run it like 70 times in 10 minutes*

## TO-DO ##
- Implement a Rust-like approach where you can just list the names of the include ( e.g. Stocksoup ) and it'll automatically fetch the URL.

## N.B ##
**This is my first Rust script. If there are any suggestions or improvements, open an issue!**
