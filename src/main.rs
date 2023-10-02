use macroquad::prelude::*;

struct Sprite {
    position: Vec2,
    texture: Texture2D,
}

impl Sprite {
    pub fn draw(&self) {
        draw_texture(
            self.texture,
            self.position.x,
            self.position.y,
            WHITE
        ); 
    }

    pub fn in_bounds(&self, position: Vec2) -> bool {
        position.x > self.position.x && 
            position.x < self.texture.width() + self.position.x &&
            position.y > self.position.y &&
            position.y < self.texture.height() + self.position.y
    }
}


#[macroquad::main("EggFeed")]
async fn main() {
    let mut eggs_eaten = 0;
    let mut is_holding_egg = false;
    let eggy_texture: Texture2D = load_texture("assets/eggy.png").await.unwrap();
    let eggy_open_texture: Texture2D = load_texture("assets/eggy_open.png").await.unwrap();
    let egg_bowl_texture: Texture2D = load_texture("assets/egg_bowl.png").await.unwrap();
    let egg: Texture2D = load_texture("assets/egg.png").await.unwrap();

    let egg_bowl: Sprite = Sprite {
        texture: egg_bowl_texture,
        position: Vec2::new(
            screen_width() - egg_bowl_texture.width() - 20.,
            screen_height() / 2.0 - egg_bowl_texture.height() / 2.
        ),
    };

    let mut egg_man: Sprite = Sprite {
        texture: eggy_texture,
        position: Vec2::new(
            20.0,
            screen_height() / 2.0 - eggy_texture.height() / 2.0
        )
    };

    loop {
        clear_background(LIGHTGRAY);
        egg_man.draw();

        egg_bowl.draw();

        let mouse_position: Vec2 = mouse_position().into();
        if egg_man.in_bounds(mouse_position) && is_holding_egg {
            egg_man.texture = eggy_open_texture;
        } else {
            egg_man.texture = eggy_texture;
        }

        if is_mouse_button_down(MouseButton::Left) {
            if egg_bowl.in_bounds(mouse_position) {
                is_holding_egg = true;
            }
        } else {
            if egg_man.in_bounds(mouse_position) && is_holding_egg {
                eggs_eaten += 1;
            }
            is_holding_egg = false;
        }

        if is_holding_egg {
            draw_texture(
                egg,
                mouse_position.x - egg.width() / 2.,
                mouse_position.y - egg.height() / 2.,
                WHITE
            );
        }

        draw_text(
            format!("Eggs Eaten {}", eggs_eaten).as_str(), 
            egg_man.position.x + 20.0, 
            egg_man.position.y + egg_man.texture.height() + 20.0,
            30.0,
            BLACK
        );

        next_frame().await
    }
}