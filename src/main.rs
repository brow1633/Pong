use macroquad::prelude::*;

struct MainState {
    top_paddle: Rect,
    bottom_paddle: Rect,
    ball: Rect,
    ball_vel: Vec2,

    top_score: usize,
    bottom_score: usize,
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
         screen_width() / 2.0,
		 15.0,
		 200.0,
		 10.0
        );
        let bottom_paddle = Rect::new(
         screen_width() / 2.0,
		 screen_height() - 30.0,
		 200.0,
		 10.0
        );

        MainState {
            top_paddle,
            bottom_paddle,
            ball,
            ball_vel: Vec2::new(5.0, -7.0),

            top_score: 0,
            bottom_score: 0,
        }
    }

    fn update(&mut self) {
        self.ball.x += self.ball_vel.x;
        self.ball.y += self.ball_vel.y;

        if self.ball.right() >= screen_width() || self.ball.left() <= 0.0 {
            self.ball_vel.x *= -1.0;
        }

        if self.ball.top() <= 0.0 || self.ball.bottom() >= screen_height() {
            self.ball_vel.y *= -1.0;
            *self = MainState::new();
        }

        if is_key_down(KeyCode::A) && self.top_paddle.left() > 0.0 {
            self.top_paddle.x -= 10.0;
        }

        if is_key_down(KeyCode::D) && self.top_paddle.right() < screen_width() {
            self.top_paddle.x += 10.0;
        }

        if is_key_down(KeyCode::Left) && self.bottom_paddle.left() > 0.0 {
            self.bottom_paddle.x -= 10.0;
        }

        if is_key_down(KeyCode::Right) && self.bottom_paddle.right() < screen_width() {
            self.bottom_paddle.x += 10.0;
        }

        /*if  self.ball.top() <= 15.0 
        && (self.ball.right() <= self.top_paddle.right()
            && self.ball.left() >= self.top_paddle.left()) {
            self.ball_vel.y *= -1.0;
        }

        if  self.ball.bottom() >= screen_height() - 30.0
        && (self.ball.right() <= self.bottom_paddle.right()
            && self.ball.left() >= self.bottom_paddle.left()) {
            self.ball_vel.y *= -1.0;
        }*/

        if self.ball.overlaps(&self.top_paddle) {
            self.ball_vel.y *= -1.0;
            self.top_score += 1;
        }

        if self.ball.overlaps(&self.bottom_paddle) {
            self.ball_vel.y *= -1.0;
            self.bottom_score += 1;
        }
    }

    fn draw(&self) {
        draw_rect(&self.ball, WHITE);
        draw_rect(&self.top_paddle, RED);
        draw_rect(&self.bottom_paddle, BLUE);
        draw_text(&self.top_score.to_string(), 15.0, 25.0, 36.0, WHITE);
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
