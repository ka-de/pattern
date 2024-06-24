# `bevy_ecs_ldtk`

---

## Customizing LDtk entities

---

The spawning of entities can be customized by registering a bundle that will be used for spawning the entity. The bundle struct must implement the `LdtkEntity` trait:

```rust
#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    #[sprite_bundle("player.png")]
    pub sprite_bundle: SpriteBundle,
    #[from_entity_instance]
    pub collider_bundle: ColliderBundle,
    pub player: Player,
    #[worldly]
    pub worldly: Worldly,
}

fn my_plugin(app: &mut App) {
    // Registers PlayerBundle to be spawned for a given Entity identifier
    app.register_ldtk_entity::<PlayerBundle>("Player");
}
```

The `LdtkEntity` trait can either be derived manually or using the derive macro as in the above example. Using derive, the fields can be components or bundles. Their instantiation can be customized using:

## `Default` trait

---

All fields must implement the `Default` trait. Implementing it manually lets you configure the initialization. This approach is relevant when:

- All usages of the component/bundle should have the same initial value,
- The initial value doesn't depend on the LDtk data of the instance.

## `#[with(...)]` and `#[from_entity_instance]`

---

These allow the field to be constructed from an [`&EntityInstance`](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/ldtk/struct.EntityInstance.html), enabling access to the data of the instance (e.g., identifier, fields, pivot, etc.).
`#[with(...)]` allows specifying a `fn (entity: &EntityInstance) -> T` user function that will construct the field `T` from the `&EntityInstance`. `#[from_entity_instance]` is similar but uses the `From<&EntityInstance>` trait of the field, which should provide a `fn from(entity: &EntityInstance) -> T` function for constructing the field.

## Special components

---

Some sprite and LDtk components have predefined instantiation methods:

- `SpriteBundle` can be initialized using the [`#[sprite_bundle(...)]`](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/app/trait.LdtkEntity.html#sprite_bundle) tag. See the player example.
- Similarly, `SpriteSheetBundle` can be initialized by specifying the [`#[sprite_sheet_bundle("path/to/asset.png", tile_width, tile_height, columns, rows, padding, offset, index)]`](https://docs.rs/bevy_ecs_ldtk/latest/bevy_ecs_ldtk/app/trait.LdtkEntity.html#sprite_sheet_bundle) tag.
- Both of the above tags will use data from the visual editor if no path or sheet coordinate are specified: `#[sprite_bundle]` and `#[sprite_sheet_bundle]`.
- `#[worldly] worldly: Worldly,` will attach a special component specifying that the entity should not be despawned when their level despawns.
- `#[grid_coords] grid_coords: GridCoords,` attach a component containing the initial grid position of the component.

## Manually implementing `LdtkEntity`

---

For advanced usage, the `LdtkEntity` trait can be manually implemented, defining the constructor function:

```rust
impl LdtkEntity for MyComponentOrBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>
    ) -> Self
    { Self { ... } }
}
```

The arguments provide everything needed for inspecting the entity (`&EntityInstance`, tile sets), the level layer (`&LayerInstance`), and loading assets (`&AssetServer` and `&mut Assets<TextureAtlas>`).
