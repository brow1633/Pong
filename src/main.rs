use macroquad::prelude::*;

struct MainState {
    top_paddle: Rect,
    top_paddle_vel: f32,
    top_target_pos: f32,
    bottom_paddle: Rect,
    ball: Rect,
    ball_vel: Vec2,

    is_multi: bool,

    score: usize,
}

impl MainState {
    fn new() -> Self {
        let ball = Rect::new(
            screen_width() / 2.0,
            screen_height() / 2.0,
            10.0,
            10.0
            );
        let top_paddle = Rect::new(
            screen_width() / 2.0 - 100.0,
            15.0,
            200.0,
            10.0
            );
        let bottom_paddle = Rect::new(
            screen_width() / 2.0 - 100.0,
            screen_height() - 15.0,
            200.0,
            10.0
            );

        MainState {
            top_paddle,
            top_paddle_vel: 0.0,
            top_target_pos: 0.0,
            bottom_paddle,
            ball,
            ball_vel: Vec2::new(5.0, 7.0),

            is_multi: false,

            score: 0,
        }
    }

    fn reset(&mut self) {
        let is_multi_copy = self.is_multi;
        *self = MainState::new();
        self.is_multi = is_multi_copy;
    }

    fn predict(&mut self, is_top: bool) {
        let time: f32;
        let time_mult: f32; //changes feel of play, for two branches must add to ~> 2.0 for it to reach ball
        if is_top {
            time = 2.0 * (self.bottom_paddle.top() - self.ball.y) / self.ball_vel.y;
            time_mult = 0.75;
        } else {
            time = (self.top_paddle.bottom() - self.ball.y) / self.ball_vel.y;
            time_mult = 1.25;
        }
        
        self.top_target_pos = self.ball_vel.x * time + self.ball.x;

        if self.top_target_pos > screen_width() {
            self.top_target_pos = screen_width() - (self.top_target_pos % screen_width());
        }

        if self.top_target_pos < 0.0 {
            self.top_target_pos *= -1.0;
        }

        self.top_paddle_vel = (self.top_target_pos - (self.top_paddle.x + 100.0)) / (time / time_mult);

    }

    fn update(&mut self) {
        self.ball.x += self.ball_vel.x;
        self.ball.y += self.ball_vel.y;

        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.ball_vel.x *= -1.0;
        }

        if self.ball.top() <= 0.0 || self.ball.bottom() >= screen_height() {
            self.ball_vel.y *= -1.0;
            self.reset();
        }

        if self.is_multi { 
            if is_key_down(KeyCode::A) && self.top_paddle.left() > 0.0 {
                self.top_paddle.x -= 10.0;
            }

            if is_key_down(KeyCode::D) && self.top_paddle.right() < screen_width() {
                self.top_paddle.x += 10.0;
            }
        } else if ((self.top_paddle_vel < 0.0 && self.top_paddle.left() > 0.0)
            |  (self.top_paddle_vel > 0.0 && self.top_paddle.right() < screen_width()))
            && (((self.top_paddle.x + 100.0)- self.top_target_pos).abs() > 5.0) {
                self.top_paddle.x += self.top_paddle_vel;
        }

        if is_key_down(KeyCode::Left) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle.x -= 10.0;
        }

        if is_key_down(KeyCode::Right) && self.bottom_paddle.right() < screen_width() {
            self.bottom_paddle.x += 10.0;
        }

        if is_key_down(KeyCode::Space) {
            self.is_multi = !self.is_multi;
        }

        if self.ball.overlaps(&self.top_paddle) {
            self.ball_vel.y *= -1.0;
            self.predict(true);
        }

        if self.ball.overlaps(&self.bottom_paddle) {
            self.ball_vel.y *= -1.0;
            self.predict(false);
            self.score += 1;
        }
    }

    fn draw(&self) {
        draw_rect(&self.ball, WHITE);
        draw_rect(&self.top_paddle, RED);
        draw_rect(&self.bottom_paddle, BLUE);
        draw_text(&self.score.to_string(), 15.0, 25.0, 36.0, WHITE);
    }
}

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        color,
        );
}

#[macroquad::main("Pong")]
async fn main() {
    let mut state = MainState::new();

    loop {
        clear_background(BLACK);

        state.update();
        state.draw();

        next_frame().await
    }
}
