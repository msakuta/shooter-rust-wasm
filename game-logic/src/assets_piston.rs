use piston_window::*;
use std::rc::Rc;

pub struct Assets {
    pub bg: Rc<G2dTexture>,
    pub weapons_tex: Rc<G2dTexture>,
    pub boss_tex: Rc<G2dTexture>,
    pub enemy_tex: Rc<G2dTexture>,
    pub spiral_enemy_tex: Rc<G2dTexture>,
    pub player_tex: Rc<G2dTexture>,
    pub shield_tex: Rc<G2dTexture>,
    pub ebullet_tex: Rc<G2dTexture>,
    pub phase_bullet_tex: Rc<G2dTexture>,
    pub spiral_bullet_tex: Rc<G2dTexture>,
    pub bullet_tex: Rc<G2dTexture>,
    pub missile_tex: Rc<G2dTexture>,
    pub explode_tex: Rc<G2dTexture>,
    pub explode2_tex: Rc<G2dTexture>,
    pub sphere_tex: Rc<G2dTexture>,
    pub power_tex: Rc<G2dTexture>,
    pub power2_tex: Rc<G2dTexture>,
}

impl Assets {
    pub fn new(window: &mut PistonWindow) -> (Self, Glyphs) {
        let mut exe_folder = std::env::current_exe().unwrap();
        exe_folder.pop();
        println!("exe: {:?}", exe_folder);
        let assets_loader = find_folder::Search::KidsThenParents(1, 3)
            .of(exe_folder)
            .for_folder("assets")
            .unwrap();

        let font = &assets_loader.join("FiraSans-Regular.ttf");
        let factory = window.factory.clone();
        let glyphs = Glyphs::new(font, factory, TextureSettings::new()).unwrap();

        let mut load_texture = |name| {
            Rc::new(
                Texture::from_path(
                    &mut window.factory,
                    &assets_loader.join(name),
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap(),
            )
        };

        (
            Self {
                bg: load_texture("back2.jpg"),
                weapons_tex: load_texture("weapons.png"),
                boss_tex: load_texture("boss.png"),
                enemy_tex: load_texture("enemy.png"),
                spiral_enemy_tex: load_texture("spiral-enemy.png"),
                player_tex: load_texture("player.png"),
                shield_tex: load_texture("shield.png"),
                ebullet_tex: load_texture("ebullet.png"),
                phase_bullet_tex: load_texture("phase-bullet.png"),
                spiral_bullet_tex: load_texture("spiral-bullet.png"),
                bullet_tex: load_texture("bullet.png"),
                missile_tex: load_texture("missile.png"),
                explode_tex: load_texture("explode.png"),
                explode2_tex: load_texture("explode2.png"),
                sphere_tex: load_texture("sphere.png"),
                power_tex: load_texture("power.png"),
                power2_tex: load_texture("power2.png"),
            },
            glyphs,
        )
    }
}
