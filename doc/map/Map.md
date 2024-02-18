# The map

By 'map', we don't mean this little paper with points on it.
The map will contain a lot of information: Technology tree, units, weapons, the **map**, and so on.

## Structure

The structure will be as followed:
```
.
├── map.chal
├── images
│   ├── fremm.jpg
│   └── ship_1.jpg
├── mods
│   └── hello.lua
└── modules
    ├── techno.chal
    ├── units.chal
    └── weapons.chal
```

## `.chal` extension

This extension is a msgpack content, allowing "easy" modification for users and developers

> See [msgpack.org](https://msgpack.org/)