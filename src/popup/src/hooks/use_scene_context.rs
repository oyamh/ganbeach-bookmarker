use domain::Scene;
use yew::prelude::*;

pub type SceneContext = UseStateHandle<Scene>;

#[hook]
pub fn use_scene_context() -> SceneContext {
    use_context::<SceneContext>().expect("no scene context found")
}

// #[derive(Debug, Clone)]
// pub struct UseSceneState(SceneContext);

// impl UseSceneState {
//     pub fn set(&self, scene: Scene) {
//         self.0.set(scene);
//     }
// }

// impl Deref for UseSceneState {
//     type Target = Scene;
//     fn deref(&self) -> &Self::Target {
//         &(*self.0)
//     }
// }

// #[hook]
// pub fn use_scene_context() -> UseSceneState {
//     log_func!();
//     let inner = use_context::<SceneContext>().expect("no scene context found");
//     UseSceneState(inner)
// }
