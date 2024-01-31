# icongen

Create icons for iOS, macOS, and watchOS with one command.

## Install

```shell
brew install icongen
```

## Generate by default

```shell
icongen --src Icon-1024.png --dst icons
```

## Generate by custom icons

- icons.yml

```yml
icons:
  Icon-88: 88
  Icon-99: 99
```

```shell
icongen --src Icon-1024.png --dst icons --icons icons.yml
```

### Output

```shell
[INFO] Generate Icon-512.png
[INFO] Generate Icon-40@3x.png
[INFO] Generate Icon-16.png
[INFO] Generate Icon-128@2x.png
[INFO] Generate Icon-64@3x.png
[INFO] Generate Icon-256.png
[INFO] Generate Icon-98@2x.png
[INFO] Generate Icon-1024.png
[INFO] Generate Icon-20.png
[INFO] Generate Icon-76.png
[INFO] Generate Icon-68@2x.png
[INFO] Generate Icon-32@2x.png
[INFO] Generate Icon-29@3x.png
[INFO] Generate Icon-40@2x.png
[INFO] Generate Icon-256@2x.png
[INFO] Generate Icon-24@2x.png
[INFO] Generate Icon-44@2x.png
[INFO] Generate Icon-512@2x.png
[INFO] Generate Icon-20@2x.png
[INFO] Generate Icon-68.png
[INFO] Generate Icon-83.5@2x.png
[INFO] Generate Icon-32.png
[INFO] Generate Icon-108@2x.png
[INFO] Generate Icon-29.png
[INFO] Generate Icon-60@2x.png
[INFO] Generate Icon-27.5@2x.png
[INFO] Generate Icon-128.png
[INFO] Generate Icon-60@3x.png
[INFO] Generate Icon-29@2x.png
[INFO] Generate Icon-64@2x.png
[INFO] Generate Icon-60.png
[INFO] Generate Icon-16@2x.png
[INFO] Generate Icon-64.png
[INFO] Generate Icon-50@2x.png
[INFO] Generate Icon-38@3x.png
[INFO] Generate Icon-20@3x.png
[INFO] Generate Icon-40.png
[INFO] Generate Icon-86@2x.png
[INFO] Generate Icon-38@2x.png
[INFO] Generate Icon-38.png
[INFO] Generate Icon-76@2x.png
[INFO] Generate successfully in directory: icons
```
