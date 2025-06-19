# voxel-inator
my dastardly tool to turn pixel art into voxels

```bash
voxel --input <input image> --output <output file name>
```
- currently only supports 1 image
- work in progress to get to 6 images to turn into a 3d voxels
- outputs a .obj and a .mtl

# wip
- need to figure out how to not have 100k different materials for each color
    - use uv mapping somehow
- need to figure out how to remove faces that aren't visible or are overlapping