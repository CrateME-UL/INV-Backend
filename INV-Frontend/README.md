# Frontend

## vscode extensions to install in general

- yzhang.markdown-all-in-one
- formulahendry.code-runner
- vsls-contrib.codetour
- wayou.vscode-todo-highlight
- streetsidesoftware.code-spell-checker
- hediet.vscode-drawio
- ms-vscode.live-server
- ms-azuretools.vscode-docker
- TabNine.tabnine-vscode
- jeff-hykin.polacode-2019
- johnpapa.vscode-peacock
- PKief.material-icon-theme
- usernamehw.errorlens
- mhutchie.git-graph
- eamodio.gitlens
- mikestead.dotenv
- aaron-bond.better-comments
- ms-vscode.notepadplusplus-keybindings
- ms-vsliveshare.vsliveshare

## vscode extensions to install for react and typescript/javascript

- WallabyJs.console-ninja
- ChakrounAnas.turbo-console-log
- dbaeumer.vscode-eslint
- dsznajder.es7-react-js-snippets
- wix.vscode-import-cost
- burkeholland.simple-react-snippets
- stylelint.vscode-stylelint
- pmneo.tsimporter
- DSKWRK.vscode-generate-getter-setter
- kisstkondoros.vscode-codemetrics
- stringham.move-ts
- WallabyJs.quokka-vscode

## to setup react with lint and prettier

https://www.robinwieruch.de/prettier-eslint/

## to setup react with typescript

https://www.robinwieruch.de/vite-typescript/

## create from template react and typescript

```bash
npm create vite@latest hacker-stories -- --template react-ts
```

## add to ./.vscode/settings.json

```json
  // enable globally (here: format on save)
  "editor.formatOnSave": true,
  // enable per-language (here: Prettier as formatter)
  "[javascript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode"
  },
  //try to fix lint on save
  "editor.codeActionsOnSave": {
    "source.fixAll.eslint": "explicit"
  },
  "eslint.validate": [
    "javascript,",
    "typescript",
    "javascriptreact",
    "typescriptreact"
  ]
```

## follow the epub the-road-to-react for for more information

## guide for scss -> make sure to code in styles.module.scss

https://sass-lang.com/guide/

## run this commmand in the styles folder in another terminal to autocompile css from change scss (install sass on computer)

```bash
sass --watch styles.module.scss styles.module.css
```

## run the test server in another terminal

```bash
npm run test
```

## to run the app server in another terminal

```bash
npm install
npm run dev
```

## to run lint/prettier: save file

## to build the docker image:

```bash
docker build -t inv-ui .
```
