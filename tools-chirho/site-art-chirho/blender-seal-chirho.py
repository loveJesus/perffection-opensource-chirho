# For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
"""The Seal: Perffection's lozenge mark as a faceted crystal with a gold core, turning once.

Rendered on a transparent film and flattened onto black, so the page can lay it over its night sky
with `mix-blend-mode: screen`: black adds no light, the crystal's highlights do. The frames become a JPEG sprite sheet that the "Secure"
section scrubs with a CSS scroll timeline (sprite-sheet-chirho.sh). No script ships to the page.

Run headless:
  /Applications/Blender.app/Contents/MacOS/Blender -b --python tools-chirho/site-art-chirho/blender-seal-chirho.py -- \
      --out-chirho target/site-art-chirho/seal-chirho/frames --frames-chirho 72 --size-chirho 900 --samples-chirho 256
"""
import argparse
import math
import sys  # Workflow: spec-chirho/workflows-chirho/first-light-authoring-chirho.md

import bmesh
import bpy
from mathutils import Vector

argv_chirho = sys.argv[sys.argv.index("--") + 1:] if "--" in sys.argv else []
parser_chirho = argparse.ArgumentParser()
parser_chirho.add_argument("--out-chirho", required=True)
parser_chirho.add_argument("--frames-chirho", type=int, default=72)
parser_chirho.add_argument("--size-chirho", type=int, default=900)
parser_chirho.add_argument("--samples-chirho", type=int, default=256)
parser_chirho.add_argument("--only-frame-chirho", type=int, default=0, help="render one frame as a still, for look development")
args_chirho = parser_chirho.parse_args(argv_chirho)

bpy.ops.wm.read_factory_settings(use_empty=True)
scene_chirho = bpy.context.scene


def ring_chirho(bm_chirho, sides_chirho, radius_chirho, z_chirho, offset_chirho=0.0):
    return [bm_chirho.verts.new((radius_chirho * math.cos(2.0 * math.pi * (index_chirho + offset_chirho) / sides_chirho),
                                 radius_chirho * math.sin(2.0 * math.pi * (index_chirho + offset_chirho) / sides_chirho),
                                 z_chirho)) for index_chirho in range(sides_chirho)]


def lozenge_mesh_chirho(name_chirho, sides_chirho, crown_z_chirho, girdle_top_chirho, girdle_bottom_chirho,
                        radius_chirho, pavilion_z_chirho):
    """An elongated two-tier cut: apex, a half-step crown ring, a narrow vertical girdle, a half-step
    pavilion ring, apex. The lozenge mark, cut so that every turn of it catches light."""
    mesh_chirho = bpy.data.meshes.new(name_chirho)
    bm_chirho = bmesh.new()
    top_chirho = bm_chirho.verts.new((0.0, 0.0, crown_z_chirho))
    bottom_chirho = bm_chirho.verts.new((0.0, 0.0, pavilion_z_chirho))
    crown_ring_chirho = ring_chirho(bm_chirho, sides_chirho, radius_chirho * 0.56, girdle_top_chirho + (crown_z_chirho - girdle_top_chirho) * 0.46, 0.5)
    upper_ring_chirho = ring_chirho(bm_chirho, sides_chirho, radius_chirho, girdle_top_chirho)
    lower_ring_chirho = ring_chirho(bm_chirho, sides_chirho, radius_chirho, girdle_bottom_chirho)
    pavilion_ring_chirho = ring_chirho(bm_chirho, sides_chirho, radius_chirho * 0.5, girdle_bottom_chirho + (pavilion_z_chirho - girdle_bottom_chirho) * 0.48, 0.5)
    for index_chirho in range(sides_chirho):
        next_chirho = (index_chirho + 1) % sides_chirho
        bm_chirho.faces.new((top_chirho, crown_ring_chirho[index_chirho], crown_ring_chirho[next_chirho]))
        bm_chirho.faces.new((crown_ring_chirho[index_chirho], upper_ring_chirho[next_chirho], crown_ring_chirho[next_chirho]))
        bm_chirho.faces.new((crown_ring_chirho[index_chirho], upper_ring_chirho[index_chirho], upper_ring_chirho[next_chirho]))
        bm_chirho.faces.new((upper_ring_chirho[index_chirho], lower_ring_chirho[index_chirho],
                             lower_ring_chirho[next_chirho], upper_ring_chirho[next_chirho]))
        bm_chirho.faces.new((lower_ring_chirho[index_chirho], pavilion_ring_chirho[index_chirho], lower_ring_chirho[next_chirho]))
        bm_chirho.faces.new((pavilion_ring_chirho[index_chirho], pavilion_ring_chirho[next_chirho], lower_ring_chirho[next_chirho]))
        bm_chirho.faces.new((bottom_chirho, pavilion_ring_chirho[next_chirho], pavilion_ring_chirho[index_chirho]))
    bmesh.ops.recalc_face_normals(bm_chirho, faces=bm_chirho.faces)
    bm_chirho.to_mesh(mesh_chirho)
    bm_chirho.free()
    object_chirho = bpy.data.objects.new(name_chirho, mesh_chirho)
    scene_chirho.collection.objects.link(object_chirho)
    for polygon_chirho in mesh_chirho.polygons:
        polygon_chirho.use_smooth = False
    return object_chirho


def glass_material_chirho():
    material_chirho = bpy.data.materials.new("seal-glass-chirho")
    material_chirho.use_nodes = True
    bsdf_chirho = material_chirho.node_tree.nodes["Principled BSDF"]
    bsdf_chirho.inputs["Base Color"].default_value = (0.94, 0.97, 1.0, 1.0)
    bsdf_chirho.inputs["Roughness"].default_value = 0.0
    bsdf_chirho.inputs["IOR"].default_value = 2.1
    bsdf_chirho.inputs["Transmission Weight"].default_value = 1.0
    if "Dispersion" in bsdf_chirho.inputs:
        bsdf_chirho.inputs["Dispersion"].default_value = 0.32
    return material_chirho


def ember_material_chirho():
    """The light inside: pure emission, warm gold, so it refracts as sparks through every facet."""
    material_chirho = bpy.data.materials.new("seal-ember-chirho")
    material_chirho.use_nodes = True
    nodes_chirho = material_chirho.node_tree.nodes
    for node_chirho in list(nodes_chirho):
        nodes_chirho.remove(node_chirho)
    emission_chirho = nodes_chirho.new("ShaderNodeEmission")
    emission_chirho.inputs["Color"].default_value = (1.0, 0.56, 0.16, 1.0)
    emission_chirho.inputs["Strength"].default_value = 5.0
    output_chirho = nodes_chirho.new("ShaderNodeOutputMaterial")
    material_chirho.node_tree.links.new(emission_chirho.outputs["Emission"], output_chirho.inputs["Surface"])
    return material_chirho


def hidden_strip_chirho(name_chirho, location_chirho, color_chirho, strength_chirho, width_chirho, height_chirho):
    """A bright card the facets reflect but the camera never sees: it is what makes glass sparkle."""
    bpy.ops.mesh.primitive_plane_add(size=1.0, location=location_chirho)
    strip_chirho = bpy.context.active_object
    strip_chirho.name = name_chirho
    strip_chirho.scale = (width_chirho, height_chirho, 1.0)
    direction_chirho = Vector((0.0, 0.0, 0.0)) - strip_chirho.location
    strip_chirho.rotation_euler = direction_chirho.to_track_quat("Z", "Y").to_euler()
    material_chirho = bpy.data.materials.new(f"{name_chirho}-light")
    material_chirho.use_nodes = True
    nodes_chirho = material_chirho.node_tree.nodes
    for node_chirho in list(nodes_chirho):
        nodes_chirho.remove(node_chirho)
    emission_chirho = nodes_chirho.new("ShaderNodeEmission")
    emission_chirho.inputs["Color"].default_value = (*color_chirho, 1.0)
    emission_chirho.inputs["Strength"].default_value = strength_chirho
    output_chirho = nodes_chirho.new("ShaderNodeOutputMaterial")
    material_chirho.node_tree.links.new(emission_chirho.outputs["Emission"], output_chirho.inputs["Surface"])
    strip_chirho.data.materials.append(material_chirho)
    strip_chirho.visible_camera = False
    return strip_chirho


def area_light_chirho(name_chirho, location_chirho, color_chirho, power_chirho, size_chirho, size_y_chirho=None):
    data_chirho = bpy.data.lights.new(name_chirho, type="AREA")
    data_chirho.energy = power_chirho
    data_chirho.color = color_chirho
    if size_y_chirho is None:
        data_chirho.size = size_chirho
    else:
        data_chirho.shape = "RECTANGLE"
        data_chirho.size, data_chirho.size_y = size_chirho, size_y_chirho
    light_chirho = bpy.data.objects.new(name_chirho, data_chirho)
    light_chirho.location = location_chirho
    direction_chirho = Vector((0.0, 0.0, 0.0)) - light_chirho.location
    light_chirho.rotation_euler = direction_chirho.to_track_quat("-Z", "Y").to_euler()
    scene_chirho.collection.objects.link(light_chirho)
    return light_chirho


# The crystal and its core.
crystal_chirho = lozenge_mesh_chirho("seal-crystal-chirho", 12, 1.32, 0.16, -0.04, 0.66, -1.42)
bevel_chirho = crystal_chirho.modifiers.new("edges-chirho", "BEVEL")
bevel_chirho.width, bevel_chirho.segments, bevel_chirho.limit_method = 0.008, 2, "ANGLE"
crystal_chirho.data.materials.append(glass_material_chirho())

core_chirho = lozenge_mesh_chirho("seal-core-chirho", 6, 0.3, 0.04, -0.02, 0.12, -0.32)
core_chirho.rotation_euler = (0.0, 0.0, math.radians(15.0))
core_chirho.data.materials.append(ember_material_chirho())

# A ring of hidden cards at different heights and colours: white and gold mostly, one cold cyan and
# one violet, so a turn throws coloured fire as well as white glints.
for index_chirho, (color_chirho, strength_chirho) in enumerate(
        [((1.0, 0.97, 0.9), 30.0), ((1.0, 0.8, 0.5), 26.0), ((0.6, 0.92, 1.0), 22.0),
         ((1.0, 0.97, 0.9), 30.0), ((0.7, 0.55, 1.0), 18.0), ((1.0, 0.85, 0.6), 24.0)]):
    angle_chirho = 2.0 * math.pi * index_chirho / 6.0 + 0.4
    hidden_strip_chirho(f"glint-{index_chirho}-chirho",
                        (3.4 * math.cos(angle_chirho), 3.4 * math.sin(angle_chirho), 1.6 if index_chirho % 2 else -0.9),
                        color_chirho, strength_chirho, 0.22, 2.6)

# A world the glass can refract but the camera never shows: film is transparent, then composited on
# black. A dim violet-to-midnight gradient gives the facets something to bend.
world_chirho = bpy.data.worlds.new("seal-world-chirho")
scene_chirho.world = world_chirho
world_chirho.use_nodes = True
nodes_chirho = world_chirho.node_tree.nodes
links_chirho = world_chirho.node_tree.links
background_chirho = nodes_chirho["Background"]
coordinate_chirho = nodes_chirho.new("ShaderNodeTexCoord")
separate_chirho = nodes_chirho.new("ShaderNodeSeparateXYZ")
ramp_chirho = nodes_chirho.new("ShaderNodeValToRGB")
ramp_chirho.color_ramp.elements[0].color = (0.004, 0.005, 0.02, 1.0)
ramp_chirho.color_ramp.elements[1].color = (0.11, 0.05, 0.16, 1.0)
links_chirho.new(coordinate_chirho.outputs["Generated"], separate_chirho.inputs["Vector"])
links_chirho.new(separate_chirho.outputs["Z"], ramp_chirho.inputs["Fac"])
links_chirho.new(ramp_chirho.outputs["Color"], background_chirho.inputs["Color"])
background_chirho.inputs["Strength"].default_value = 0.6

# Gold key from upper left, cold rim behind right, a long top strip for one crisp reflection line,
# and a low violet fill so the pavilion is never dead.
area_light_chirho("key-chirho", (-3.6, -3.2, 3.4), (1.0, 0.84, 0.58), 900.0, 2.4)
area_light_chirho("rim-chirho", (3.4, 3.6, 1.6), (0.56, 0.89, 1.0), 1400.0, 1.6)
area_light_chirho("strip-chirho", (0.0, -1.6, 4.6), (1.0, 0.97, 0.92), 380.0, 4.0, 0.22)
area_light_chirho("fill-chirho", (1.2, -3.8, -2.4), (0.48, 0.36, 1.0), 420.0, 2.2)

camera_data_chirho = bpy.data.cameras.new("camera-chirho")
camera_data_chirho.lens = 85.0
camera_chirho = bpy.data.objects.new("camera-chirho", camera_data_chirho)
camera_chirho.location = (0.0, -8.6, 0.55)
camera_chirho.rotation_euler = (math.radians(87.0), 0.0, 0.0)
scene_chirho.collection.objects.link(camera_chirho)
scene_chirho.camera = camera_chirho

# One full turn of the crystal; the core turns half a turn the other way, so the light inside moves
# against the light outside.
frames_chirho = args_chirho.frames_chirho
scene_chirho.frame_start, scene_chirho.frame_end = 1, frames_chirho
for frame_chirho in range(1, frames_chirho + 1):
    progress_chirho = (frame_chirho - 1) / frames_chirho
    crystal_chirho.rotation_euler = (math.radians(4.0), 0.0, 2.0 * math.pi * progress_chirho)
    crystal_chirho.keyframe_insert("rotation_euler", frame=frame_chirho)
    core_chirho.rotation_euler = (0.0, 0.0, math.radians(15.0) - math.pi * progress_chirho)
    core_chirho.keyframe_insert("rotation_euler", frame=frame_chirho)
# A key on every frame, so interpolation never matters and the turn is exactly even.

render_chirho = scene_chirho.render
render_chirho.engine = "CYCLES"
render_chirho.resolution_x = render_chirho.resolution_y = args_chirho.size_chirho
render_chirho.film_transparent = True
render_chirho.image_settings.file_format = "PNG"
render_chirho.image_settings.color_mode = "RGBA"
scene_chirho.view_settings.view_transform = "AgX"
for look_chirho in ("AgX - Punchy", "Punchy"):
    try:
        scene_chirho.view_settings.look = look_chirho
        break
    except TypeError:
        continue
cycles_chirho = scene_chirho.cycles
cycles_chirho.device = "GPU"
cycles_chirho.samples = args_chirho.samples_chirho
cycles_chirho.use_denoising = True
cycles_chirho.max_bounces = 24
cycles_chirho.transmission_bounces = 20
cycles_chirho.glossy_bounces = 12
cycles_chirho.caustics_reflective = True
cycles_chirho.caustics_refractive = True
preferences_chirho = bpy.context.preferences.addons["cycles"].preferences
preferences_chirho.compute_device_type = "METAL"
preferences_chirho.get_devices()
for device_chirho in preferences_chirho.devices:
    device_chirho.use = device_chirho.type == "METAL" and "M5" in device_chirho.name

# Frames are RGBA with a transparent film; sprite-sheet-chirho.sh flattens them onto exact black, so
# the JPEG background adds no light under the page's screen blend.
if args_chirho.only_frame_chirho:
    scene_chirho.frame_set(args_chirho.only_frame_chirho)
    render_chirho.filepath = f"{args_chirho.out_chirho}/still-{args_chirho.only_frame_chirho:04d}-chirho.png"
    bpy.ops.render.render(write_still=True)
else:
    render_chirho.filepath = f"{args_chirho.out_chirho}/frame-####-chirho"
    bpy.ops.render.render(animation=True)
print("SEAL-RENDER-DONE-CHIRHO")
