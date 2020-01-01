# Sliceform Surfaces (2019)

A sliceform kirigami generator made by Peter Gagliardi

This code is still experimental, and will probably change greatly over time.

## How it works

This program takes a mathematical surface, slices it up, and produces a
PostScript template that can be printed on cardstock, cut out, and assembled
into a model.

The process works like this:
1. Define a surface, `z = f(x, y)` and pass it to the slicer
2. The slicer computes traces of the surface along the x-axis and y-axis. It
    produces some polygons with the resulting shape, and marks places where the
    slits go.
3. The printer takes these polygons and arranges them in a PostScript file. it
    packs the polygons onto the page 

## Usage

About that... I haven't gotten around to adding command line arguments yet. 
Edit `src/main.rs` if you want to change the model type and resolution.
The models are listed in `src/models.rs` for now.

Once the code is configured, do the following:

```
# Run the program, generating slicetest.ps
cargo run

# Either use the PostScript file directly, or if you have GhostScript installed,
# you can run this command to generate slicetest.pdf
ps2pdf slicetest.ps
```
