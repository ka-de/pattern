/// this code is compiled only if debug assertions are enabled (debug mode)
use bevy::prelude::*;

use bevy::input::common_conditions::input_toggle_active;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub(crate) fn plugin(app: &mut bevy::app::App) {
    
    app.add_plugins((
        // FrameTimeDiagnosticsPlugin
        bevy::diagnostic::FrameTimeDiagnosticsPlugin,
        // LogDiagnosticsPlugin
        bevy::diagnostic::LogDiagnosticsPlugin::default(),
        // EntityCountDiagnosticsPlugin
        bevy::diagnostic::EntityCountDiagnosticsPlugin::default(),
        // SystemInformationDiagnosticsPlugin
        bevy::diagnostic::SystemInformationDiagnosticsPlugin::default(),
        // WorldInspectorPlugin
        WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F11)),
        // PerformanceUI
        crate::components::perfui::setup_perf_ui,
    ));
}

mod log_mod {
    use bevy::log::tracing_subscriber::Layer;
    use bevy::log::tracing_subscriber::prelude::*;
    use bevy::log::BoxedSubscriber;
    use bevy::utils::tracing::Subscriber;

    pub struct MyLayer {
        // ...
    }

    impl<S: Subscriber> Layer<S> for MyLayer {
        fn register_callsite(
            &self,
            metadata: &'static bevy::utils::tracing::Metadata<'static>
        ) -> bevy::utils::tracing::subscriber::Interest {
            println!("register_callsite: {:#?}", metadata);
            bevy::utils::tracing::subscriber::Interest::always()
        }
        // ...
    }

    pub fn update_subscriber(subscriber: BoxedSubscriber) -> BoxedSubscriber {
        Box::new(subscriber.with(MyLayer {}))
    }
}

pub(crate) fn make_log_plugin() -> impl Plugin {
    bevy::log::LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,pattern=debug,wgpu_core=warn,wgpu_hal=warn,pattern=debug".into(),
        update_subscriber: Some(log_mod::update_subscriber),
    }
}
