use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 640;
const SCREEN_HEIGHT: i32 = 480;
const PADDLE_SPEED: f32 = 5.0;
const BALL_SPEED: f32 = 5.0;

const SCREEN_CENTRE: Vector2 = Vector2 { x: (SCREEN_WIDTH / 2) as f32, y: (SCREEN_HEIGHT / 2) as f32 };

struct Ball {
    rect: Rectangle,
    color: Color,
    speed: Vector2,
}

impl Ball {
    fn new(rect: Rectangle, color: Color, speed: Vector2) -> Self {
        Self { rect, color, speed }
    }

    fn get_center(&self) -> Vector2 {
        Vector2 { x: self.rect.x + (self.rect.width / 2.0), y: self.rect.y + (self.rect.height / 2.0) }
    }
    
    fn apply_move(&mut self) {
        self.rect.x += self.speed.x;
        self.rect.y += self.speed.y;
    }

    fn bounce_off_wall(&mut self) {
        self.speed.y *= -1.0;
    }

    fn bounce_off_paddle(&mut self, paddle: Rectangle) {
        let paddle_center = Vector2::new(paddle.x + (paddle.width / 2.0), paddle.y + (paddle.height / 2.0));
        let direction_to_paddle = (self.get_center() - paddle_center).normalized();     
        self.speed = direction_to_paddle * BALL_SPEED;
    }

    fn reset(&mut self, point_left: bool) {
        self.rect.x = SCREEN_CENTRE.x - self.rect.width / 2.0;
        self.rect.y = SCREEN_CENTRE.y - self.rect.height / 2.0;

        if point_left {
            self.speed = Vector2 { x: BALL_SPEED, y: 0.0 };
        } else {
            self.speed = Vector2 { x: -BALL_SPEED, y: 0.0};
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("PONG")
        .build();
     
    rl.set_target_fps(60);
    let mut paddle_one = Rectangle::new(16.0, 16.0, 16.0, 96.0);
    let mut paddle_two = Rectangle::new(SCREEN_WIDTH as f32 - 32.0, 16.0, 16.0, 96.0);
    let mut ball = Ball::new(
        Rectangle::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0, 16.0, 16.0),
        Color::WHITE,
        Vector2::new(-BALL_SPEED, 0.0) );

    let mut score_left = 0;
    let mut score_right = 0;

    let mut is_game_over = false;
    let mut is_serve = true;
    
    while !rl.window_should_close() {

        if is_game_over {
        
            let mut d = rl.begin_drawing(&thread);

            let left_width = measure_text("LEFT WINS!", 72);
            let right_width = measure_text("RIGHT WINS!", 72);

            d.clear_background(Color::DARKGRAY);
            if score_left == 9 {
                d.draw_text("LEFT WINS!", SCREEN_CENTRE.x as i32 - (left_width / 2), SCREEN_CENTRE.y as i32, 72, Color::RED);
            }

            if score_right == 9 {
                d.draw_text("RIGHT WINS!", SCREEN_CENTRE.x as i32 - (right_width / 2), SCREEN_CENTRE.y as i32, 72, Color::RED);
            }

        } else {

            if rl.is_key_down(KeyboardKey::KEY_W) && paddle_one.y > 1.0 {
               paddle_one.y -= PADDLE_SPEED; 
            }

            if rl.is_key_down(KeyboardKey::KEY_S) && paddle_one.y < (SCREEN_HEIGHT as f32 - paddle_one.height){
                paddle_one.y += PADDLE_SPEED;
            }

            if rl.is_key_down(KeyboardKey::KEY_I) && paddle_two.y > 1.0 {
               paddle_two.y -= PADDLE_SPEED; 
            }

            if rl.is_key_down(KeyboardKey::KEY_K) && paddle_two.y < (SCREEN_HEIGHT as f32 - paddle_two.height){
                paddle_two.y += PADDLE_SPEED;
            }

            if !is_serve {
                ball.apply_move();
            }

            if rl.is_key_down(KeyboardKey::KEY_SPACE){
                is_serve = false;
            }
            
            if check_collision(&paddle_one, &ball.rect) {
                ball.bounce_off_paddle(paddle_one);   
            }

            if check_collision(&paddle_two, &ball.rect) {
                ball.bounce_off_paddle(paddle_two);
            }

            if ball.rect.y <= 0.0 || (ball.rect.y + ball.rect.height) >= SCREEN_HEIGHT as f32 {
                ball.bounce_off_wall();
            }

            if ball.rect.x >= SCREEN_WIDTH as f32 {
                score_left += 1;
                ball.reset(true);
                is_serve = true;
            }

            if ball.rect.x <= 0.0 {
                score_right += 1;
                ball.reset(false);
                is_serve = true;
            }

            if score_left == 9 || score_right == 9 {
                is_game_over = true;
            }

            let mut d = rl.begin_drawing(&thread);

                    
            d.clear_background(Color::BLACK);
            d.draw_text(format!("{} | {}", score_left, score_right).as_str(), (SCREEN_WIDTH / 2) - 32, 16, 32, Color::GRAY);
            d.draw_rectangle_rec(paddle_one, Color::WHITE);
            d.draw_rectangle_rec(paddle_two, Color::WHITE);
            d.draw_rectangle_rec(ball.rect, ball.color);
        }
    }
}

fn check_collision(rectangle1: &Rectangle, rectangle2: &Rectangle) -> bool {

    //collsion on x-axis?
    let collision_x = rectangle1.x + rectangle1.width >= rectangle2.x &&
    rectangle2.x + rectangle2.width >= rectangle1.x;
    //collision on y-axis?
    let collision_y = rectangle1.y + rectangle1.height >= rectangle2.y &&
    rectangle2.y + rectangle2.height >= rectangle1.y;

    collision_x && collision_y
}

