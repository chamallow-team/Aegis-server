# Map

## Saving & Loading a map

A map will be saved like this:
```
name;description
[MAP:length_in_bytes]
...bytes...
[WEAPONS:length_in_bytes]
...bytes...
```

You got the concept.

### Map

The map will be stored as a msgpack information for nodes & edges

Here is an example:
```json
{
  "nodes": [
    {
      "id": "c4c91f4c-d82b-4fd4-a68b-2d7a8a2eac57",
      "node_type": "Water",
      "coordinates": [1, 2]
    },
    {
      "id": "b6ae2af9-4627-49a2-b379-6b35a379c2a4",
      "node_type": "Water",
      "coordinates": [4, 5]
    },
    {
      "id": "738be122-9698-4b1c-8b32-3ec1eca49e11",
      "node_type": "Water",
      "coordinates": [6, 9]
    }
  ],
  "edges": [
    {
      "source_id": "c4c91f4c-d82b-4fd4-a68b-2d7a8a2eac57",
      "target_id": "b6ae2af9-4627-49a2-b379-6b35a379c2a4"
    },
    {
      "source_id": "b6ae2af9-4627-49a2-b379-6b35a379c2a4",
      "target_id": "738be122-9698-4b1c-8b32-3ec1eca49e11"
    }
  ]
}
```

And his msgpack version (88% less datas):
```
nodes\x93\xa2id\xd9$c4c91f4c-d82b-4fd4-a68b-2d7a8a2eac57\xa9node_type\xa5Water\xabcoordinates\x92\x01\x02\xa2id\xd9$b6ae2af9-4627-49a2-b379-6b35a379c2a4\xa9node_type\xa5Water\xabcoordinates\x92\x04\x05\xa2id\xd9$738be122-9698-4b1c-8b32-3ec1eca49e11\xa9node_type\xa5Water\xabcoordinates\x92\x06\t\xa5edges\x92\x82\xa9source_id\xd9$c4c91f4c-d82b-4fd4-a68b-2d7a8a2eac57\xa9target_id\xd9$b6ae2af9-4627-49a2-b379-6b35a379c2a4\xa9source_id\xd9$b6ae2af9-4627-49a2-b379-6b35a379c2a4\xa9target_id\xd9$738be122-9698-4b1c-8b32-3ec1eca49e11
```