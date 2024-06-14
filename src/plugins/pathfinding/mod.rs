use bevy::{
    app::{ App, Update },
    asset::{ Assets, Handle },
    ecs::{ query::Added, system::{ Query, Res } },
    log,
};
use bevy_ecs_ldtk::{ assets::LdtkProject, prelude::RawLevelAccessor };

pub fn plugin(app: &mut App) {
    app.add_systems(Update, process_ldtk_project);
}

/// Converts the int cell layer of the levels into representation for pathfinder (WIP)
pub(crate) fn process_ldtk_project(
    ldtk_project_handle_query: Query<&Handle<LdtkProject>, Added<Handle<LdtkProject>>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>
) {
    let Ok(ldtk_project_handle) = ldtk_project_handle_query.get_single() else {
        return;
    };
    let ldtk_project = ldtk_project_assets
        .get(ldtk_project_handle)
        .expect("Project should be loaded if level has spawned");
    for level in ldtk_project.iter_raw_levels() {
        log::debug!("level {} ({})", level.identifier, level.iid);
        let Some(layers) = &level.layer_instances else {
            continue;
        };
        const LKT: &[u8] = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
        for layer in layers {
            let layer_string = if !layer.int_grid_csv.is_empty() {
                format!(
                    ":\n{}",
                    layer.int_grid_csv
                        .chunks(layer.c_wid as usize)
                        .map(|c|
                            String::from_utf8(
                                c
                                    .iter()
                                    .map(|i| LKT[*i as usize])
                                    .collect::<Vec<_>>()
                            ).unwrap()
                        )
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            } else {
                String::new()
            };
            log::debug!(
                "layer {} ({}) {}x{} v:{} {}",
                layer.identifier,
                layer.iid,
                layer.c_wid,
                layer.c_hei,
                layer.visible,
                layer_string
            );
        }
    }
}
