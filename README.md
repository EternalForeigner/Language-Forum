# Development

Your new jumpstart project includes basic organization with an organized `assets` folder and a `components` folder. 
If you chose to develop with the router feature, you will also have a `views` folder.

### Tailwind
1. Install tailwind: https://aur.archlinux.org/packages/tailwindcss-bin
2. Run:

```bash
tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve
```

To run for a different platform, use the `--platform platform` flag. E.g.
```bash
dx serve --platform desktop
```

### Autostruct
To generate the types:
autostruct .env file: `DATABASE_URL=postgres://postgres:postgres@127.0.0.1:54322/postgres`
Command:
`./autostruct generate --singular --use-statements "use serde::{Deserialize, Serialize};" --derive-statements "Deserialize,Serialize,Clone,PartialEq"`
