# c8-ox-www

## WASM Implementation

This has not been tested in a real environment yet. For a first glance use these instructions.

Build the WASM file and JS module.
```
wasm-pack build
```

Switch to the webpage folder.
```
cd www
```

Start the development server (webpack).
```
npm run start
```

Navigate to your browser and open up [localhost:8080](localhost:8080) on your loopback address. The page should now be displayed.

You will see a prompt for the ROM path. The base directory for this is the `roms` directory in the `www` Folder. One example for a valid input would be

`chip8/TETRIS`

### Sources

- Square Wave 440 Hz sound from YouTube since I was too lazy to record it myself: [440Hz](https://www.youtube.com/watch?v=G10qLid60xw)
